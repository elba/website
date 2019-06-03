import React, { useCallback, useState, useEffect } from "react"
import history from "~/history"
import style from "./styles.scss"
import {
  PackageView,
  PackageReq,
  show_package,
  download_stats,
  VersionReq,
} from "~api"
import { RemoteData } from "~utils/remote-data"
import { Link } from "react-router-dom"

export const Package: React.FunctionComponent<PackageReq> = props => {
  const [packageView, setPackageView] = useState<RemoteData<PackageView>>({
    type: "Not Asked",
  })
  const [downloads, setDownloads] = useState<RemoteData<number>>({
    type: "Not Asked",
  })

  useEffect(() => {
    if (packageView.type === "Not Asked") {
      load()
    }
  })

  const load = async () => {
    setPackageView({ type: "Started" })
    let packageView = await show_package(props)
    setPackageView({ type: "Done", data: packageView })

    let version_req: VersionReq = {
      group: packageView.latest_version.group,
      package: packageView.latest_version.package,
      version: packageView.latest_version.version,
    }
    setDownloads({ type: "Started" })
    let stats = await download_stats(version_req)
    setDownloads({ type: "Done", data: stats.total })
  }

  const onPackageClick = () => {
    if (packageView.type === "Done") {
      history.push({
        pathname: `/package/${packageView.data.latest_version.group}/${
          packageView.data.latest_version.package
        }/${packageView.data.latest_version.version}`,
      })
    }
  }

  const onTagClick = (tag: string) => {
    if (packageView.type === "Done") {
      history.push({
        pathname: `/search?q=${tag}`,
      })
    }
  }

  if (packageView.type === "Done") {
    return (
      <div className={style["package-item"]}>
        <div className={style["title-row"]} onClick={onPackageClick}>
          <span className={style["title"]}>
            {packageView.data.group} / {packageView.data.package}
          </span>
          <span className={style["version"]}>
            {packageView.data.latest_version.version}
          </span>
        </div>
        <span className={style["description"]}>
          {packageView.data.latest_version.description}
        </span>
        <div className={style["tag-container"]}>
          {packageView.data.latest_version.keywords.map((keyword, idx) => (
            <Link
              to={`/search?q=${keyword}`}
              key={idx}
              className={style["tag"]}
            >
              {keyword}
            </Link>
          ))}
        </div>
        <div className={style["bottom-row"]}>
          <img
            className={style["avatar"]}
            src={packageView.data.latest_version.owners[0].avatar}
            alt="avatar"
          />
          <span className={style["author"]}>
            {packageView.data.latest_version.owners[0].name}
          </span>
          <span className={style["separator"]}>•</span>
          <span className={style["last-updated"]}>
            {new Date(packageView.data.updated_at).toDateString()}
          </span>
        </div>
        <b className={style["downloads-counter-number"]}>
          {downloads.type === "Done" ? downloads.data : "-"}
        </b>
        <p className={style["downloads-counter-title"]}>downloads</p>
      </div>
    )
  } else {
    return (
      <div className={[style["placeholder"], style["package-item"]].join(" ")}>
        <div className={style["title-row"]}>
          <span className={style["title"]}>
            {props.group} / {props.package}
          </span>
        </div>
        <div className={style["placeholder__bars"]}>
          <div />
          <div />
          <div />
        </div>
      </div>
    )
  }
}

type PackageListProps = {
  packages: PackageReq[]
}

export const PackageList: React.FunctionComponent<PackageListProps> = props => (
  <div className={style.listing}>
    {props.packages.map(item => (
      <Package key={`${item.group}/${item.package}`} {...item} />
    ))}
  </div>
)
