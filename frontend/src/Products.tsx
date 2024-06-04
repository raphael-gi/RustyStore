import { Suspense, useEffect, useState } from "react"
import { Card, CardDescription, CardTitle } from "./components/ui/card";

type Product = {
  id: number,
  name: string,
  description: string,
  price: number
}

function Products() {
  const [products, setProducts] = useState<Product[]|undefined>(undefined);

  useEffect(() => {
      const jwt: string = localStorage.getItem("jwt") || "";
      fetch('/api/products', {
          headers: {
              Authorization: jwt
          }
      })
      .then((res) => res.json())
      .then((data: Product[]) => setProducts(data));
  }, []);

  return (
    <main>
      <h1>Rusty Store</h1>
      <Suspense fallback={<h3>Loading...</h3>}>
        <div className="mx-[5%] grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
          {products?.map((product: Product) =>
            <Card className="p-10">
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

export default Products
