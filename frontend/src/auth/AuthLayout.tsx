import { Card } from "@/components/ui/card";
import { Toaster } from "@/components/ui/toaster";
import { cn } from "@/lib/utils";
import { Outlet } from "react-router-dom";

function AuthLayout() {
  return (
    <main className="h-screen grid place-content-center">
      <Card className={cn("w-[550px]")}>
        <Outlet />
        <Toaster />
      </Card>
    </main>
  )
}

export default AuthLayout;
