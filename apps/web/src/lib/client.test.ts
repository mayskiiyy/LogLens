import { describe, it, expect } from "vitest";
import { HttpLogLensClient } from "./client/HttpClient";

describe("HttpLogLensClient", () => {
  it("should instantiate successfully", () => {
    const client = new HttpLogLensClient();
    expect(client).toBeDefined();
  });
});
