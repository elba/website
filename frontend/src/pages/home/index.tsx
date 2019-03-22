import React from "react"
import style from "./styles.scss"
import { PackageList, PackageView } from "~/components/package_list"
import avatar from '~/img/avatar.jpg';

const testResults: PackageView[] = new Array(5).fill({
    group: "lightyear",
    name: "lightyear",
    version: "0.2.1",
    description:
        "Lightweight parser combinator library for Idris, inspired by Parsec.",
    keywords: ["parser", "parser", "parser", "parser", "parser"],
    downloads: 102,
    avatar: avatar,
    author: "ziman",
    updated_at: new Date(),
})

export const Homepage: React.FunctionComponent = () => (
    <div className={style.page}>
        <Statistics />
        <section className={style["section"]}>
            <h2 className={style["section-title"]}>Popular packages</h2>
            <PackageList packages={testResults} />
        </section>
    </div>
)


export const Statistics: React.FunctionComponent = () => (
    <div className={[style["section"], style["statistics"]].join(" ")}>
        <h2 className={style["section-title"]}>Until now, we have</h2>
        <ul className={style["statistics-item-container"]}>
            <li className={style["statistics-item"]}>
                <p className={style["statistics-value"]}>1230</p>
                <p className={style["statistics-title"]}>packages</p>
            </li>
            <li className={style["statistics-item"]}>
                <p className={style["statistics-value"]}>201649</p>
                <p className={style["statistics-title"]}>downloads</p>
            </li>
        </ul>
    </div>
)
