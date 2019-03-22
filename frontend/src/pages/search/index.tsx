import React from "react"
import style from "./styles.scss"

type PackageView = {
    group: string
    name: string
    version: string
    description: string
    keywords: string[]
    downloads: number
    updated_at: Date
}

const testResults: PackageView[] = new Array(5).fill({
    group: "lightyear",
    name: "lightyear",
    version: "0.2.1",
    description:
        "Lightweight parser combinator library for Idris, inspired by Parsec.",
    keywords: ["parser", "parser", "parser", "parser", "parser"],
    downloads: 102,
    updated_at: new Date(),
})
console.log(style)

export const SearchResultsPage: React.FunctionComponent = () => (
    <div className={style.page}>
        <header className={style.header}>
            <b>Search result</b> for{" "}
            <span className={style["search-term"]}>lightyear</span>
        </header>
        <main>
            <div className={style["listing-top-bar"]}>
                <span>100 packages found</span>
                <div>
                    {[1, 2, 3, 4, 5, 200].map(pageNumber => (
                        <button key={pageNumber}>{pageNumber}</button>
                    ))}
                </div>
            </div>
            <div className={style.listing}>
                {testResults.map((item, idx) => (
                    <div key={idx} className={style["listing-item"]}>
                        <div className={style["listing-item__title-row"]}>
                            <span className={style["listing-item__title"]}>
                                {item.group} / {item.name}
                            </span>
                            <span className={style["listing-item__version"]}>
                                {item.version}
                            </span>
                        </div>
                        <span className={style["listing-item__description"]}>
                            {item.description}
                        </span>
                        <div className={style["listing-item__tag-container"]}>
                            {item.keywords.map((keyword, idx) => (
                                <span
                                    key={idx}
                                    className={style["listing-item__tag"]}
                                >
                                    {keyword}
                                </span>
                            ))}
                        </div>
                        <div className={style["listing-item__bottom-row"]}>
                            <span className={style["listing-item__last-updated"]}>
                                {item.updated_at.toDateString()}
                            </span>
                        </div>
                        <span className={style["listing-item__downloads-counter"]}>
                            <b>{item.downloads}</b>
                            <p>downloads</p>
                        </span>
                    </div>
                ))}
            </div>
        </main>
    </div>
)
