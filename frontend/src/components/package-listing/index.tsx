import React, { useCallback } from "react"
import history from "~/history"
import style from "./styles.scss"

export type PackageProps = {
  group: string
  name: string
  version: string
  description: string
  keywords: string[]
  downloads: number
  avatar: string
  author: string
  updatedAt: Date
  onClick?: React.MouseEventHandler<HTMLDivElement>
}

export const Package: React.FunctionComponent<PackageProps> = props => {
  const clickHandler = useCallback(() => {
    history.push({
      pathname: `/package/${props.group}/${props.name}`,
    })
  }, [props.group, props.name])
  return (
    <div
      className={style["package-item"]}
      onClick={props.onClick || clickHandler}
    >
      <div className={style["title-row"]}>
        <span className={style["title"]}>
          {props.group}/{props.name}
        </span>
        <span className={style["version"]}>{props.version}</span>
      </div>
      <span className={style["description"]}>{props.description}</span>
      <div className={style["tag-container"]}>
        {props.keywords.map((keyword, idx) => (
          <a key={idx} className={style["tag"]}>
            {keyword}
          </a>
        ))}
      </div>
      <div className={style["bottom-row"]}>
        <img className={style["avatar"]} src={props.avatar} alt="avatar" />
        <span className={style["author"]}>{props.author}</span>
        <span className={style["separator"]}>•</span>
        <span className={style["last-updated"]}>
          {props.updatedAt.toDateString()}
        </span>
      </div>
      <span className={style["downloads-counter"]}>
        <b>{props.downloads}</b>
        <p>downloads</p>
      </span>
    </div>
  )
}

type PackageListProps = {
  packages: PackageProps[]
}

export const PackageList: React.FunctionComponent<PackageListProps> = props => (
  <div className={style.listing}>
    {props.packages.map(item => (
      <Package key={`${item.group}/${item.name}`} {...item} />
    ))}
  </div>
)
