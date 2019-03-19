import React from "react"
import s from "./styles.scss"

type PackageSearchResultItem = {
    group: string
    name: string
    version: string
    description: string
    tags: string[]
    downloads: number
    updated_at: Date
}

const testResults: PackageSearchResultItem[] = new Array(5).fill({
    group: "lightyear",
    name: "lightyear",
    version: "0.2.1",
    description:
        "Lightweight parser combinator library for Idris, inspired by Parsec.",
    tags: ["parser", "parser", "parser", "parser"],
    downloads: 102,
    updated_at: new Date(),
})
console.log(s)

export const SearchResultsPage: React.FunctionComponent = () => (
    <div className={s.page}>
        <header className={s.header}>
            <b>Search result</b> for{" "}
            <span className={s["search-term"]}>lightyear</span>
        </header>
        <main>
            <div className={s["listing-top-bar"]}>
                <span>100 packages found</span>
                <div>
                    {[1, 2, 3, 4, 5, 200].map(pageNumber => (
                        <button key={pageNumber}>{pageNumber}</button>
                    ))}
                </div>
            </div>
            <div className={s.listing}>
                {testResults.map((r, idx) => (
                    <div key={idx} className={s["listing-item"]}>
                        <span className={s["listing-item__title"]}>
                            {r.group} / {r.name}
                        </span>
                        <span className={s["listing-item__version"]}>
                            {r.version}
                        </span>
                        <span className={s["listing-item__downloads_counter"]}>
                            {r.downloads} downloads
                        </span>
                        <span className={s["listing-item__description"]}>
                            {r.description}
                        </span>
                        <div className={s["listing-item__tag_container"]}>
                            {r.tags.map((t, idx) => (
                                <span
                                    key={idx}
                                    className={s["listing-item__tag"]}
                                >
                                    {t}
                                </span>
                            ))}
                        </div>
                        <span className={s["listing-item__last-updated"]}>
                            {r.updated_at.toDateString()}
                        </span>
                    </div>
                ))}
            </div>
        </main>
    </div>
)
