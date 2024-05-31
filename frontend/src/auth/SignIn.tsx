import { Button } from "@/components/ui/button";
import { CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Form, FormControl, FormField, FormItem, FormLabel } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

type FormData = {
  username: string,
  password: string
}

function SignIn() {
  const navigate = useNavigate();

  const form = useForm({
    defaultValues: {
      username: "",
      password: ""
    }
  });

  const handleSubmit = async (data: FormData) => {
    const bodyData = new URLSearchParams(data);

    try {
      const res = await fetch("/api/login", {
        method: "POST",
        body: bodyData
      });

      if (res.ok) {
        const jwt = await res.text();
        console.log(jwt);
        navigate("/");
      } else {
        const message = await res.text();
        console.error(message);
      }
    } catch (err) {
      console.error(err);
    }
  }

  return (
    <>
      <CardHeader>
        <CardTitle className="text-center">Sign In</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(handleSubmit)} className="space-y-3">

            <FormField
              control={form.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Username</FormLabel>
                  <FormControl>
                    <Input
                      required
                      minLength={3}
                      maxLength={90}
                      placeholder="Username"
                      {...field} />
                  </FormControl>
                </FormItem>
              )} />

            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input
                      required
                      type="password"
                      minLength={8}
                      placeholder="Password"
                      {...field} />
                  </FormControl>
                </FormItem>
              )} />

            <Button className="w-full" type="submit">Sign In</Button>
          </form>
        </Form>
      </CardContent>
      <CardFooter><p>New here? Sign up <a href="/auth/register">here</a></p></CardFooter>
    </>
  )
}

export default SignIn
