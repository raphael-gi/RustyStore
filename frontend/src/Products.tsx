import { Suspense } from "react"
import { Card, CardDescription, CardTitle } from "./components/ui/card";
import { useNavigate } from "react-router-dom";
import { useQuery } from "react-query";

type Product = {
  id: number,
  name: string,
  description: string,
  price: number
}

function Products() {
  const navigate = useNavigate();

  const { data, error } = useQuery<Product[], Error>({
    queryKey: ["products"],
    queryFn: async () => {
      const jwt: string = localStorage.getItem("jwt") || "";
      const res = await fetch('/api/products', {
        headers: {
          Authorization: jwt
        }
      })
      if (!res.ok) {
        const message = await res.text();
        throw new Error(message);
      }
      return res.json();
    }
  })

  if (error) return 'An error has occurred: ' + error.message

  return (
    <main>
      <h1 className="text-6xl text-center my-10">Rusty Store</h1>
      <Suspense fallback={<h3>Loading...</h3>}>
        <div className="mx-[5%] grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
          {data?.map((product: Product) =>
            <Card className="p-10 cursor-pointer" onClick={() => navigate(`/product/${product.id}`)}>
              <CardTitle>{product.name}</CardTitle>
              <CardDescription className="mt-2">
                {product.description}
              </CardDescription>
            </Card>
          )}
        </div>
      </Suspense>
    </main>
  )
}

export default Products;
