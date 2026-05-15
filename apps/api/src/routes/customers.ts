import { Router } from "express";

export const customersRouter = Router();

// In-memory store — replace with DB in production
const submissions = new Map<string, { dataHash: string; status: string; timestamp: number }>();

customersRouter.post("/submit", (req, res) => {
  const { address, dataHash } = req.body as { address: string; dataHash: string };
  if (!address || !dataHash) {
    return res.status(400).json({ error: "address and dataHash required" });
  }
  submissions.set(address, { dataHash, status: "pending", timestamp: Date.now() });
  return res.json({ success: true, address, status: "pending" });
});

customersRouter.post("/validate", (req, res) => {
  const { address, approved } = req.body as { address: string; approved: boolean };
  const record = submissions.get(address);
  if (!record) return res.status(404).json({ error: "customer not found" });
  record.status = approved ? "validated" : "rejected";
  return res.json({ success: true, address, status: record.status });
});

customersRouter.get("/:address", (req, res) => {
  const record = submissions.get(req.params.address);
  if (!record) return res.status(404).json({ error: "not found" });
  return res.json({ address: req.params.address, ...record });
});
