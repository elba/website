import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { PackageList } from "~components/package-listing"
import { RemoteData } from "~/utils/remote-data"
import queryString from "query-string"
import { Redirect } from "react-router"
import { PackageReq, search } from "~api"

type LocationProps = {
  location: { search: string }
}

type State = {
  result: RemoteData<PackageReq[]>
}

export class SearchResultsPage extends React.Component<LocationProps, State> {
  constructor(props: LocationProps) {
    super(props)
    this.state = {
      result: {
        type: "Not Asked",
      },
    }
  }

  async load() {
    const query = queryString.parse(this.props.location.search)
    let search_results = await search(query.q as string)
    this.setState({
      result: { type: "Done", data: search_results },
    })
  }

  componentDidMount() {
    if (this.state.result.type !== "Done") this.load()
  }

  componentDidUpdate(prevProps: LocationProps) {
    if (
      this.state.result.type !== "Done" ||
      prevProps.location.search != this.props.location.search
    )
      this.load()
  }

  render() {
    const query = queryString.parse(this.props.location.search)
    if ((query.q || "") === "") {
      return <Redirect to="/" />
    }

    return (
      <div className={style.page}>
        <header className={style.header}>
          <span className={style["search-result"]}>Search results</span>
          <span className={style["search-for"]}>for</span>
          <span className={style["search-term"]}>{query.q}</span>
        </header>
        <main>
          <div className={style["listing-top-bar"]}>
            <span className={style["packages-found-label"]}>
              {this.state.result.type === "Done"
                ? `${this.state.result.data.length} packages found`
                : "Loading"}
            </span>
          </div>
          {this.state.result.type === "Done" ? (
            <PackageList packages={this.state.result.data} />
          ) : (
            <p />
          )}
        </main>
      </div>
    )
  }
}

export default SearchResultsPage
