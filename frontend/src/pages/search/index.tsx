import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { PackageList } from "~components/package-listing"
import { RemoteData } from "~/utils/remote-data"
import queryString from "query-string"
import { Redirect } from "react-router"
import { PackageReq, search } from "~api"
import { Navbar } from "~components/navbar"

type LocationProps = {
  location: { search: string }
}

export const SearchResultsPage: React.FunctionComponent<
  LocationProps
> = props => {
  const [result, setResult] = useState<RemoteData<PackageReq[]>>({
    type: "Not Asked",
  })

  const query = queryString.parse(props.location.search)
  const q = (query.q as string) || ""

  if (q === "") {
    return <Redirect to="/" />
  }

  useEffect(() => {
    const load = async () => {
      let search_results = await search(q)
      setResult({
        type: "Done",
        data: search_results,
      })
    }
    load()
  }, [props.location.search])

  return (
    <div>
      <Navbar />
      <div className={style.page}>
        <header className={style.header}>
          <p className={style["search-result"]}>Search results</p>
          <p className={style["search-for"]}>for</p>
          <p className={style["search-term"]}>{query.q}</p>
        </header>
        <main>
          <div className={style["listing-top-bar"]}>
            <span className={style["packages-found-label"]}>
              {result.type === "Done"
                ? `${result.data.length} packages found`
                : "Loading"}
            </span>
          </div>
          {result.type === "Done" ? (
            <PackageList packages={result.data} />
          ) : (
            undefined
          )}
        </main>
      </div>
    </div>
  )
}

export default SearchResultsPage
