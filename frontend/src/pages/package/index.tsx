import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import avatar from "~/img/avatar.jpg"
import ReadmeViewer from "~/components/readme-viewer"
import {
  VersionReq,
  VersionView,
  show_version,
  show_readme,
  list_versions,
} from "~api"
import { RemoteData } from "~utils/remote-data"
import { Link } from "react-router-dom"
import { Navbar } from "~components/navbar"

type ParamProps = {
  match: { params: { group: string; package: string; version: string } }
}

export const PackageDetailsPage: React.FunctionComponent<
  ParamProps
> = props => {
  const [versionView, setVersionView] = useState<RemoteData<VersionView>>({
    type: "Not Asked",
  })
  const [versions, setVersions] = useState<RemoteData<VersionReq[]>>({
    type: "Not Asked",
  })
  const [readme, setReadme] = useState<RemoteData<string>>({
    type: "Not Asked",
  })

  useEffect(() => {
    load()
  }, [props.match.params])

  const load = async () => {
    let version_req = {
      group: props.match.params.group,
      package: props.match.params.package,
      version: props.match.params.version,
    }

    show_version(version_req).then(version_view =>
      setVersionView({ type: "Done", data: version_view })
    )

    list_versions({
      group: version_req.group,
      package: version_req.package,
    }).then(versions_list => setVersions({ type: "Done", data: versions_list }))

    show_readme(version_req).then(readme_string =>
      setReadme({ type: "Done", data: readme_string })
    )
  }

  return (
    <div>
      <Navbar />
      {versionView.type === "Done" ? (
        <div className={style.page}>
          <header className={style["title"]}>
            <span className={style["title__name"]}>
              {versionView.data.group} / {versionView.data.package}
            </span>
            <span className={style["title__version"]}>
              {versionView.data.version}
            </span>
          </header>
          <div className={style["package-top-bar"]}>
            {versionView.data.homepage ? (
              <a href={"//" + versionView.data.homepage} target="_blank">
                Homepage
              </a>
            ) : (
              undefined
            )}
            {versionView.data.repository ? (
              <a href={"//" + versionView.data.repository} target="_blank">
                Repository
              </a>
            ) : (
              undefined
            )}
          </div>
          <div className={style["main-layout"]}>
            <main>
              {readme.type === "Done" ? (
                <div className={style["main-layout__readme"]}>
                  <ReadmeViewer markdown={readme.data} />
                </div>
              ) : (
                undefined
              )}
            </main>
            <aside className={style["main-layout__info"]}>
              <div className={style["main-layout__info__item"]}>
                <p>Install</p>
                <pre>
                  "{versionView.data.group}/{versionView.data.package}" ={" "}
                  {versionView.data.version}
                </pre>
              </div>
              {versionView.data.license ? (
                <div className={style["main-layout__info__item"]}>
                  <p>License</p>
                  <a className={style["item-link"]}>
                    {versionView.data.license}
                  </a>
                </div>
              ) : (
                undefined
              )}
              <div className={style["main-layout__info__item"]}>
                <p>Versions</p>
                {versions.type === "Done"
                  ? versions.data.map(version =>
                      version.version === versionView.data.version ? (
                        <span
                          key={version.version}
                          className={[
                            style["item-link"],
                            style["disabled"],
                          ].join(" ")}
                        >
                          {version.version}
                        </span>
                      ) : (
                        <Link
                          key={version.version}
                          className={style["item-link"]}
                          to={`/package/${version.group}/${version.package}/${
                            version.version
                          }`}
                        >
                          {version.version}
                        </Link>
                      )
                    )
                  : undefined}
              </div>
              <div className={style["main-layout__info__item"]}>
                <p>Owners</p>
                {versionView.data.owners.map(owner => (
                  <div
                    className={style["main-layout__owner"]}
                    key={owner.email}
                  >
                    {owner.avatar ? (
                      <img
                        className={style["owner__avatar"]}
                        src={owner.avatar}
                        alt="avatar"
                      />
                    ) : (
                      undefined
                    )}
                    <div className={style["owner__text"]}>
                      <div className={style["owner__name"]}>{owner.name}</div>
                      <div className={style["owner__email"]}>{owner.email}</div>
                    </div>
                  </div>
                ))}
              </div>
            </aside>
          </div>
        </div>
      ) : (
        undefined
      )}
    </div>
  )
}

export default PackageDetailsPage
