import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  vus: __ENV.VUS || 150, // Virtual Users - exceeds queue capacity of 100
  duration: __ENV.DURATION || "30s",
};

const BASE_URL = __ENV.BASE_URL || "http://localhost:3000";

// Replace with a valid flash_sale_id from your database
const FLASH_SALE_ID =
  __ENV.FLASH_SALE_ID || "11111111-1111-1111-1111-111111111111";

export function setup() {
  // Fetch users to use in the test
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

  const url = `${BASE_URL}/orders`;
  const userId = data.userIds[Math.floor(Math.random() * data.userIds.length)];

  const payload = JSON.stringify({
    user_id: userId,
    flash_sale_id: FLASH_SALE_ID,
    quantity: 1,
  });

  const params = {
    headers: {
      "Content-Type": "application/json",
    },
  };

  const res = http.post(url, payload, params);

  check(res, {
    "is valid response": (r) => [201, 409, 404, 429, 503].includes(r.status),
    "201 = created": (r) => r.status === 201,
    "409 = sold out": (r) => r.status === 409,
    "404 = not found": (r) => r.status === 404,
    "429 = rate limited": (r) => r.status === 429,
    "503 = queue overflow": (r) => r.status === 503,
  });

  // No sleep - maximize pressure to test admission control
}
