import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  vus: 10, // Virtual Users
  duration: "30s",
};

// Replace with a valid flash_sale_id from your database
const FLASH_SALE_ID = "00000000-0000-0000-0000-000000000001";
const BASE_URL = __ENV.BASE_URL || "http://localhost:3000";

export default function () {
  const url = `${BASE_URL}/purchase`;
  const payload = JSON.stringify({
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
