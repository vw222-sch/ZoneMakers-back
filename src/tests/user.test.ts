import { describe, it, expect } from "vitest";

const BASE = "http://localhost:3000";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async function register(username: string, email: string, password = "password123") {
  return fetch(`${BASE}/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, email, password, region: 1 }),
  });
}

// ---------------------------------------------------------------------------
// POST /register
// ---------------------------------------------------------------------------

describe("POST /register", () => {
  it("returns 200 with a token and id for a new user", async () => {
    const res = await register("alice", "alice@test.com");

    expect(res.status).toBe(200);

    const body = await res.json();
    expect(body).toHaveProperty("token");
    expect(body).toHaveProperty("id");
    expect(typeof body.token).toBe("string");
    expect(typeof body.id).toBe("number");
  });

  it("returns 409 when the username or email is already taken", async () => {
    await register("bob", "bob@test.com"); // first — should succeed
    const res = await register("bob", "bob@test.com"); // duplicate

    expect(res.status).toBe(409);
    const body = await res.json();
    expect(body).toHaveProperty("error");
  });
});

// ---------------------------------------------------------------------------
// GET /user/:id
// ---------------------------------------------------------------------------

describe("GET /user/:id", () => {
  it("returns the user that was just registered", async () => {
    // Register to get a known id.
    const regRes = await register("carol", "carol@test.com");
    const { id } = await regRes.json();

    const res = await fetch(`${BASE}/user/${id}`);

    expect(res.status).toBe(200);

    const body = await res.json();
    expect(body.id).toBe(id);
    expect(body.username).toBe("carol");
    expect(body).toHaveProperty("handle");
    expect(body).toHaveProperty("reputation");
    // Sensitive fields should not be exposed.
    expect(body).not.toHaveProperty("password");
    expect(body).not.toHaveProperty("email");
  });

  it("returns 404 for a user id that does not exist", async () => {
    const res = await fetch(`${BASE}/user/999999`);

    expect(res.status).toBe(404);
    const body = await res.json();
    expect(body).toHaveProperty("error");
  });
});
