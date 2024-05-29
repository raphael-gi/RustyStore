import { Button } from "@/components/ui/button";
import { CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Form, FormControl, FormField, FormItem, FormLabel } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useForm } from "react-hook-form";

function SignIn() {
  const form = useForm();

  return (
    <>
      <CardHeader>
        <CardTitle className="text-center">Sign In</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form className="space-y-3">

            <FormField
              control={form.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Username or Email</FormLabel>
                  <FormControl>
                    <Input
                      required
                      minLength={3}
                      maxLength={90}
                      placeholder="Username or Email"
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
