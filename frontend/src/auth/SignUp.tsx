import { Button } from "@/components/ui/button";
import { CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Form, FormControl, FormField, FormItem, FormLabel } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useToast } from "@/components/ui/use-toast";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

type FormData = {
  username: string,
  email: string,
  password: string
}

function SignUp() {
  const navigate = useNavigate();
  const { toast } = useToast();

  const form = useForm({
    defaultValues: {
      username: "",
      email: "",
      password: ""
    }
  });

  const handleSubmit = async (data: FormData) => {
    const bodyData = new URLSearchParams(data);

    try {
      const res = await fetch("/api/register", {
        method: "POST",
        body: bodyData
      })
      if (res.ok) {
        toast({
          title: `Hi ${data.username}, your account has been created`,
          description: `An email has been sent to ${data.email} please authenticate your account`
        })
        navigate("/auth");
      } else {
        console.error(res);
      }
    } catch (err) {
      console.error(err);
    }
  }

  return (
    <>
      <CardHeader>
        <CardTitle className="text-center">Sign Up</CardTitle>
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
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Email</FormLabel>
                  <FormControl>
                    <Input
                      type="email"
                      required
                      maxLength={90}
                      placeholder="Email"
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
                      type="password"
                      required
                      minLength={8}
                      placeholder="Password"
                      {...field} />
                  </FormControl>
                </FormItem>
              )} />

            <Button className="w-full" type="submit">Sign Up</Button>
          </form>
        </Form>
      </CardContent>
      <CardFooter><p>Already have an account? Sign in <a href="/auth">here</a></p></CardFooter>
    </>
  )
}

export default SignUp;
