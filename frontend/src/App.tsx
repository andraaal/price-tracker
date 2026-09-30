import { useEffect, useState } from "react";
import type { Product } from "./types";
import ProductCard from "./ProductCard";

type SortOption = "PriceAsc" | "PriceDesc" | "NameAsc" | "NameDesc" | "RefAsc" | "RefDesc";

const vendors = ["Spar"];

const tags = [
  "Organic",
  "Vegan",
  "Vegetarian",
  "GlutenFree",
  "LactoseFree",
  "FairTrade",
  "AMASigil",
  "Cooled",
  "Frozen",
];

function toEuro(cents: number) {
  return (cents / 100).toFixed(2) + " €";
}

function App() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(1);
  const [pageLength, setPageLength] = useState(20);

  const [search, setSearch] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [vendorFilter, setVendorFilter] = useState("");
  const [tagFilter, setTagFilter] = useState("");
  const [maxPrice, setMaxPrice] = useState("");
  const [sortBy, setSortBy] = useState<SortOption>("PriceAsc");

  useEffect(() => {
    setLoading(true);

    const params = new URLSearchParams({
      page: String(page),
      page_length: String(pageLength),
    });

    if (searchQuery.trim()) {
      params.set("search_string", searchQuery.trim());
    }

    if (vendorFilter) {
      params.set("vendor", vendorFilter);
    }

    if (tagFilter) {
      params.set("tag", tagFilter);
    }

    const parsedMaxPrice = Number(maxPrice.replace(",", "."));

    if (Number.isFinite(parsedMaxPrice) && parsedMaxPrice > 0) {
      params.set("max_price", String(Math.round(parsedMaxPrice * 100)));
    }

    params.set("sort", sortBy);

    fetch(`/api/products?${params}`)
      .then((r) => {
        if (!r.ok) {
          throw new Error("Failed to fetch products");
        }
        return r.json();
      })
      .then((data) => {
        setProducts(data);
        setLoading(false);
      })
      .catch(() => {
        setProducts([]);
        setLoading(false);
      });
  }, [page, pageLength, searchQuery, vendorFilter, tagFilter, maxPrice, sortBy]);

  const resetPage = () => setPage(1);

  const hasPreviousPage = page > 1;
  const hasNextPage = products.length === pageLength;

  const pageStart = Math.max(1, page - 2);
  const pageEnd = hasNextPage ? page + 2 : page;
  const pageItems = Array.from(
    { length: pageEnd - pageStart + 1 },
    (_, index) => pageStart + index,
  );

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
          <div className="col-12 col-sm-6 col-lg-2">
            <label htmlFor="page-length" className="form-label mb-1">
              Page size
            </label>
            <select
              id="page-length"
              className="form-select"
              value={pageLength}
              onChange={(e) => {
                setPage(1);
                setPageLength(Number(e.target.value));
              }}
            >
              <option value={12}>12</option>
              <option value={20}>20</option>
              <option value={36}>36</option>
              <option value={48}>48</option>
            </select>
          </div>

          <div className="col-12 col-lg-4">
            <label htmlFor="search-input" className="form-label mb-1">
              Search
            </label>
            <input
              id="search-input"
              className="form-control"
              placeholder="Product, brand..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  setSearchQuery(search);
                  resetPage();
                }
              }}
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
              onChange={(e) => {
                setVendorFilter(e.target.value);
                resetPage();
              }}
            >
              <option value="">All vendors</option>
              {vendors.map((vendor) => (
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
              onChange={(e) => {
                setTagFilter(e.target.value);
                resetPage();
              }}
            >
              <option value="">All tags</option>
              {tags.map((tag) => (
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
              onChange={(e) => {
                setSortBy(e.target.value as SortOption);
                resetPage();
              }}
            >
              <option value="PriceAsc">Price: low to high</option>
              <option value="PriceDesc">Price: high to low</option>
              <option value="NameAsc">Name: A-Z</option>
              <option value="NameDesc">Name: Z-A</option>
              <option value="RefAsc">Reference price: low to high</option>
              <option value="RefDesc">Reference price: high to low</option>
            </select>
          </div>
        </div>
      </div>

      <div className="d-flex align-items-center justify-content-between mb-3">
        <h2 className="h5 mb-0">Products</h2>
        <small className="text-muted">
          Page {page} · Showing {products.length}
        </small>
      </div>

      {products.length === 0 ? (
        <div className="alert alert-secondary">No products match the current filters.</div>
      ) : (
        <div className="row">
          {products.map((product) => (
            <div className="col-12 col-md-6 col-xl-4 mb-4" key={product.id}>
              <ProductCard product={product} />
            </div>
          ))}
        </div>
      )}

      {products.length > 0 && (
        <p className="text-muted small mb-0">
          Cheapest current product: <strong>{toEuro(products[0].price)}</strong>
        </p>
      )}

      <nav aria-label="Products pages" className="mt-3">
        <ul className="pagination justify-content-center mb-0">
          <li className={`page-item ${hasPreviousPage ? "" : "disabled"}`}>
            <button
              type="button"
              className="page-link"
              onClick={() => setPage((currentPage) => Math.max(1, currentPage - 1))}
              disabled={!hasPreviousPage}
            >
              Previous
            </button>
          </li>

          {pageItems.map((pageNumber) => (
            <li key={pageNumber} className={`page-item ${pageNumber === page ? "active" : ""}`}>
              <button type="button" className="page-link" onClick={() => setPage(pageNumber)}>
                {pageNumber}
              </button>
            </li>
          ))}

          <li className={`page-item ${hasNextPage ? "" : "disabled"}`}>
            <button
              type="button"
              className="page-link"
              onClick={() => setPage((currentPage) => currentPage + 1)}
              disabled={!hasNextPage}
            >
              Next
            </button>
          </li>
        </ul>
      </nav>
    </div>
  );
}

export default App;
