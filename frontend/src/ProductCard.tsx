import type { Product } from "./types";

type Props = {
  product: Product;
};

function formatPrice(cents: number) {
  return (cents / 100).toFixed(2) + " €";
}

function formatReference(product: Product) {
  const { price, unit } = product.reference;
  return `${formatPrice(price)} / ${unit}`;
}

export default function ProductCard({ product }: Props) {
  return (
    <div className="card h-100 border-0 shadow-sm">
      <img
        src={product.image_url}
        className="card-img-top bg-light"
        alt={product.name}
        style={{ height: "220px", objectFit: "contain", padding: "1rem" }}
      />

      <div className="card-body d-flex flex-column">
        <div className="d-flex align-items-center justify-content-between mb-2">
          <small className="text-muted text-uppercase">{product.vendor}</small>
          <span className="badge text-bg-light border">{product.brand}</span>
        </div>

        <h5 className="card-title mb-2">{product.name}</h5>

        <p className="text-muted mb-2">{product.quantity}</p>
        <div className="mb-2">
          {product.tags.map((tag) => (
            <span key={tag} className="badge text-bg-secondary me-1 mb-1">
              {tag}
            </span>
          ))}
        </div>

        <div className="mt-auto pt-2 border-top">
          <h4 className="mb-1">{formatPrice(product.price)}</h4>
          <p className="small text-muted mb-0">
            Reference: <strong>{formatReference(product)}</strong>
          </p>
        </div>
      </div>
    </div>
  );
}
