import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  vus: 10, // Virtual Users
  duration: "30s",
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
    "is status 201 or 409 or 404": (r) => [201, 409, 404].includes(r.status),
  });

  sleep(0.1); // Small sleep to control request rate
}
