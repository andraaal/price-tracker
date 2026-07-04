function Footer() {
  return (
    <>
      <div className="container pb-4" id="impressum">
        <section className="p-4 rounded-4 bg-light border">
          <h2 className="h4 mb-3">Impressum</h2>

          <div className="mb-4">
            <h3 className="h6 text-uppercase text-muted mb-2">Externe Inhalte</h3>
            <p className="mb-0 text-body-secondary">
              Auf dieser Seite werden externe Inhalte von Drittanbietern eingebunden. Diese Inhalte
              koennen urheberrechtlich geschuetzt sein. Die Nutzung dieser Inhalte erfolgt nur zu
              privaten Nutzung und in Einklang mit den geltenden Urheberrechtsgesetzen. Inhaber der
              Rechte an den Inhalten Dritter koennen sich bei etwaigen Urheberrechtsverletzungen
              ueber die unten angegebene E-Mail Addresse melden. Die bemaengelten Inhalte werden
              dann umgehend entfernt.
            </p>
          </div>

          <div>
            <h3 className="h6 text-uppercase text-muted mb-2">Kontakt</h3>
            <p className="mb-0">
              E-Mail: <a href="mailto:draaal@fantasymail.de">draaal@fantasymail.de</a>
            </p>
          </div>
        </section>
      </div>

      <div className="container">
        <footer className="d-flex flex-wrap justify-content-between align-items-center py-3 my-4 border-top">
          <div className="col-md-4 d-flex align-items-center">
            <span className="mb-3 mb-md-0 text-body-secondary">© 2026 Andreas Muthenthaler</span>
          </div>
        </footer>
      </div>
    </>
  );
}

export default Footer;
