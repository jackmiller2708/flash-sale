import http from "k6/http";
import { check, sleep, fail } from "k6";

export const options = {
  vus: __ENV.VUS || 150,
  duration: __ENV.DURATION || "30s",
};

const BASE_URL = __ENV.BASE_URL || "http://localhost:3000";
const FLASH_SALE_ID =
  __ENV.FLASH_SALE_ID || "11111111-1111-1111-1111-111111111111";

// Polling config
const MAX_POLL_ATTEMPTS = 10;
const POLL_INTERVAL_SEC = 1;

export function setup() {
  const res = http.get(`${BASE_URL}/users`);
  const users = res.json();

  if (!users || users.length === 0) {
    console.error("No users found! Please create some users first.");
    return { userIds: [] };
  }

  return { userIds: users.map((u) => u.id) };
}

export default function (data) {
  if (data.userIds.length === 0) {
    return;
  }

  const res = http.post(
    `${BASE_URL}/orders`,
    JSON.stringify({
      user_id: data.userIds[Math.floor(Math.random() * data.userIds.length)],
      flash_sale_id: FLASH_SALE_ID,
      quantity: 1,
    }),
    { headers: { "Content-Type": "application/json" } },
  );

  check(res, {
    "valid response": (r) => [202, 409, 404, 429, 503].includes(r.status),
    "202 = accepted": (r) => r.status === 202,
    "409 = sold out": (r) => r.status === 409,
    "404 = not found": (r) => r.status === 404,
    "429 = rate limited": (r) => r.status === 429,
    "503 = queue overflow": (r) => r.status === 503,
  });

  // Only poll if async order was accepted
  if (res.status !== 202) {
    sleep(0.1);
    return;
  }

  const body = res.json();
  const orderId = body.order_id;

  // Poll order status
  for (let attempt = 1; attempt <= MAX_POLL_ATTEMPTS; attempt++) {
    sleep(POLL_INTERVAL_SEC);

    const statusRes = http.get(`${BASE_URL}/orders/${orderId}/status`, {
      tags: { endpoint: "order_status" },
    });

    const status = statusRes.json("status");

    // Terminal state reached
    if (status !== "pending") {
      check(statusRes, {
        "status request ok": (r) => r.status === 200,
        "terminal status": () => ["completed", "failed"].includes(status),
      });

      return;
    }
  }

  // If we reach here, polling timed out
  fail("Order still pending after max polling attempts");
}
