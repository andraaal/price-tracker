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
        className="card-img-top bg-light position-relative"
        alt={product.name}
        style={{
          height: "220px",
          objectFit: "contain",
          paddingTop: "0.5rem",
          paddingBottom: "1.5rem",
        }}
      />
      <a
        href={product.shop_url}
        style={{ top: "195px", left: "50%", transform: "translate(-50%, 0)", position: "absolute" }}
      >
        From {product.vendor}
      </a>

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
          <h4 className="mb-1">
            {formatPrice(product.price)}
            {product.base_price < product.price ? (
              <span className="text-decoration-line-through text-muted ms-2">
                {formatPrice(product.base_price)}
              </span>
            ) : null}
          </h4>
          <p className="small text-muted mb-0">
            Reference: <strong>{formatReference(product)}</strong>
          </p>
        </div>
      </div>
    </div>
  );
}
