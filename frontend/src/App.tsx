import { useEffect, useMemo, useState } from "react";
import type { Product } from "./types";
import ProductCard from "./ProductCard";

type SortOption = "price-asc" | "price-desc" | "name-asc" | "reference-asc";

function toEuro(cents: number) {
  return (cents / 100).toFixed(2) + " €";
}

function App() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState("");
  const [vendorFilter, setVendorFilter] = useState("all");
  const [tagFilter, setTagFilter] = useState("all");
  const [maxPrice, setMaxPrice] = useState("");
  const [sortBy, setSortBy] = useState<SortOption>("price-asc");

  useEffect(() => {
    fetch("/api/products")
      .then((r) => r.json())
      .then((data) => {
        setProducts(data);
        setLoading(false);
      });
  }, []);

  const vendorOptions = useMemo(
    () =>
      [...new Set(products.map((p) => p.vendor))]
        .filter(Boolean)
        .sort((a, b) => a.localeCompare(b)),
    [products],
  );

  const tagOptions = useMemo(
    () =>
      [...new Set(products.flatMap((p) => p.tags))]
        .filter(Boolean)
        .sort((a, b) => a.localeCompare(b)),
    [products],
  );

  const filteredProducts = useMemo(() => {
    const normalizedSearch = search.trim().toLowerCase();
    const parsedMaxPrice = Number(maxPrice.replace(",", "."));
    const maxPriceInCents =
      Number.isFinite(parsedMaxPrice) && parsedMaxPrice > 0
        ? Math.round(parsedMaxPrice * 100)
        : Number.NaN;

    const matches = products.filter((product) => {
      const matchesSearch =
        normalizedSearch.length === 0 ||
        [product.name, product.brand, product.vendor, product.quantity, ...product.tags]
          .join(" ")
          .toLowerCase()
          .includes(normalizedSearch);

      const matchesVendor = vendorFilter === "all" || product.vendor === vendorFilter;
      const matchesTag = tagFilter === "all" || product.tags.includes(tagFilter);
      const matchesMaxPrice = Number.isNaN(maxPriceInCents) || product.price <= maxPriceInCents;

      console.log(
        product.name,
        matchesSearch,
        matchesVendor,
        matchesTag,
        matchesMaxPrice,
        product.price,
        maxPriceInCents,
        parsedMaxPrice,
      );

      return matchesSearch && matchesVendor && matchesTag && matchesMaxPrice;
    });

    return matches.sort((a, b) => {
      switch (sortBy) {
        case "price-desc":
          return b.price - a.price;
        case "name-asc":
          return a.name.localeCompare(b.name);
        case "reference-asc":
          return a.reference.price - b.reference.price;
        case "price-asc":
        default:
          return a.price - b.price;
      }
    });
  }, [maxPrice, products, search, sortBy, tagFilter, vendorFilter]);

  if (loading) {
    return <div className="container mt-4">Loading...</div>;
  }

  return (
    <div className="container py-4">
      <div className="p-4 rounded-4 bg-light border mb-4">
        <h1 className="mb-2">Price Tracker</h1>
        <p className="text-muted mb-4">
          Search, compare and filter products by vendor, tag and price.
        </p>

        <div className="row g-3">
          <div className="col-12 col-lg-4">
            <label htmlFor="search-input" className="form-label mb-1">
              Search
            </label>
            <input
              id="search-input"
              className="form-control"
              placeholder="Product, brand, vendor, tag..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>

          <div className="col-12 col-sm-6 col-lg-2">
            <label htmlFor="vendor-filter" className="form-label mb-1">
              Vendor
            </label>
            <select
              id="vendor-filter"
              className="form-select"
              value={vendorFilter}
              onChange={(e) => setVendorFilter(e.target.value)}
            >
              <option value="all">All vendors</option>
              {vendorOptions.map((vendor) => (
                <option key={vendor} value={vendor}>
                  {vendor}
                </option>
              ))}
            </select>
          </div>

          <div className="col-12 col-sm-6 col-lg-2">
            <label htmlFor="tag-filter" className="form-label mb-1">
              Tag
            </label>
            <select
              id="tag-filter"
              className="form-select"
              value={tagFilter}
              onChange={(e) => setTagFilter(e.target.value)}
            >
              <option value="all">All tags</option>
              {tagOptions.map((tag) => (
                <option key={tag} value={tag}>
                  {tag}
                </option>
              ))}
            </select>
          </div>

          <div className="col-12 col-sm-6 col-lg-2">
            <label htmlFor="max-price" className="form-label mb-1">
              Max price (€)
            </label>
            <input
              id="max-price"
              className="form-control"
              inputMode="decimal"
              placeholder="e.g. 2.99"
              value={maxPrice}
              onChange={(e) => setMaxPrice(e.target.value)}
            />
          </div>

          <div className="col-12 col-sm-6 col-lg-2">
            <label htmlFor="sort-filter" className="form-label mb-1">
              Sort by
            </label>
            <select
              id="sort-filter"
              className="form-select"
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
            >
              <option value="price-asc">Price: low to high</option>
              <option value="price-desc">Price: high to low</option>
              <option value="name-asc">Name: A-Z</option>
              <option value="reference-asc">Reference price: low to high</option>
            </select>
          </div>
        </div>
      </div>

      <div className="d-flex align-items-center justify-content-between mb-3">
        <h2 className="h5 mb-0">Products</h2>
        <small className="text-muted">
          Showing {filteredProducts.length} of {products.length}
        </small>
      </div>

      {filteredProducts.length === 0 ? (
        <div className="alert alert-secondary">No products match the current filters.</div>
      ) : (
        <div className="row">
          {filteredProducts.map((product) => (
            <div className="col-12 col-md-6 col-xl-4 mb-4" key={product.id}>
              <ProductCard product={product} />
            </div>
          ))}
        </div>
      )}

      {filteredProducts.length > 0 && (
        <p className="text-muted small mb-0">
          Cheapest current product: <strong>{toEuro(filteredProducts[0].price)}</strong>
        </p>
      )}
    </div>
  );
}

export default App;
