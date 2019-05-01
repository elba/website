import React from "react"
import marked from "marked"

export type ReadmeProps = {
  markdown: string
}

export const ReadmeViewer: React.FunctionComponent<ReadmeProps> = props => {
  return (
    <div
      dangerouslySetInnerHTML={{
        __html: marked(props.markdown),
      }}
    />
  )
}

export default ReadmeViewer
