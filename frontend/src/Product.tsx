import { useQuery } from "react-query";
import { useParams } from "react-router-dom"

type Product = {
  id: number,
  name: string,
  description: string,
  price: number
}

function Product() {
  const { id } = useParams();

  const { data, error } = useQuery<Product, Error>({
    queryKey: ["product"],
    queryFn: async () => {
      const res = await fetch(`/api/product/${id}`);
      if (!res.ok) {
        const message = await res.text();
        throw new Error(message);
      }
      return res.json();
    }
  })

  if (error) return 'An error has occurred: ' + error.message

  return (
    <div>
      <h1 className="text-6xl">{data?.name}</h1>
      <h2>{data?.price}</h2>
      <h3>{data?.description}</h3>
    </div>
  )
}

export default Product
