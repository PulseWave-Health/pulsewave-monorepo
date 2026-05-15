import express from "express";
import cors from "cors";
import { customersRouter } from "./routes/customers";

const app = express();
app.use(cors());
app.use(express.json());

app.use("/customers", customersRouter);

app.get("/health", (_req, res) => res.json({ status: "ok" }));

const PORT = process.env.PORT ?? 3001;
app.listen(PORT, () => console.log(`API running on port ${PORT}`));
