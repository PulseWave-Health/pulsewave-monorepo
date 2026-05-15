import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Pulsewave",
  description: "Customer validation pipeline",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
