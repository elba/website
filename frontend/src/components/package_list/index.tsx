import React from "react"
import style from "./styles.scss"

export type PackageView = {
    group: string
    name: string
    version: string
    description: string
    keywords: string[]
    downloads: number
    avatar: string
    author: string
    updated_at: Date
}

type PackageViewProps = {
    packages: PackageView[],
}

export const PackageList: React.FunctionComponent<PackageViewProps> = (props) => (
    <div className={style.listing}>
        {props.packages.map((item, idx) => (
            <div key={idx} className={style["listing-item"]}>
                <div className={style["title-row"]}>
                    <span className={style["title"]}>
                        {item.group}/{item.name}
                    </span>
                    <span className={style["version"]}>
                        {item.version}
                    </span>
                </div>
                <span className={style["description"]}>
                    {item.description}
                </span>
                <div className={style["tag-container"]}>
                    {item.keywords.map((keyword, idx) => (
                        <a key={idx} className={style["tag"]}>
                            {keyword}
                        </a>
                    ))}
                </div>
                <div className={style["bottom-row"]}>
                    <img className={style["avatar"]} src={item.avatar} alt="avatar"></img>
                    <span className={style["author"]}>
                        {item.author}
                    </span>
                    <span className={style["separator"]}>
                        •
                    </span>
                    <span className={style["last-updated"]}>
                        {item.updated_at.toDateString()}
                    </span>
                </div>
                <span className={style["downloads-counter"]}>
                    <b>{item.downloads}</b>
                    <p>downloads</p>
                </span>
            </div>
        ))}
    </div>
)
