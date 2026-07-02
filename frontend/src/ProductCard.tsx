import type { Product } from "./types";

type Props = {
  product: Product;
};

function formatPrice(cents: number) {
  return (cents / 100).toFixed(2) + " €";
}

export default function ProductCard({ product }: Props) {
  return (
    <div className="card h-100 shadow-sm">
      <img
        src={product.image_url}
        className="card-img-top"
        alt={product.name}
        style={{ height: "220px", objectFit: "contain", padding: "1rem" }}
      />

      <div className="card-body d-flex flex-column">
        <small className="text-muted">{product.vendor}</small>

        <h5 className="card-title">{product.name}</h5>

        <p className="mb-1">{product.brand}</p>

        <p className="text-muted">{product.quantity}</p>
        <div className="mb-2">
          {product.tags.map((tag) => (
            <span key={tag} className="badge bg-secondary me-1">
              {tag}
            </span>
          ))}
        </div>

        <div className="mt-auto">
          <h4>{formatPrice(product.price)}</h4>
        </div>
      </div>
    </div>
  );
}
