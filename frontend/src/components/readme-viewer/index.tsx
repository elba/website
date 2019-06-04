import React from "react"
import style from "./styles.scss"
import marked from "marked"

export type ReadmeProps = {
  markdown: string
}

export const ReadmeViewer: React.FunctionComponent<ReadmeProps> = props => {
  return (
    <div
      className={style["markdown"]}
      dangerouslySetInnerHTML={{
        __html: marked(props.markdown),
      }}
    />
  )
}

export default ReadmeViewer
