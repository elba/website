import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { TokenList, TokenProps } from "~components/token-list"
import { RemoteData } from "~/utils/remote-data"

const testResults: TokenProps[] = new Array(5)
  .fill({
    token_partial: "34243....234234",
    created_at: new Date(),
  })
  .map((item, idx) => ({ ...item, token_id: idx }))

export const UserProfilePage: React.FunctionComponent = () => {
  const [result, setResult] = useState<TokenProps[]>([])
  useEffect(() => {
    if (result.length === 0) {
      setTimeout(() => {
        setResult(testResults)
      }, 1000)
    }
  })
  return (
    <div className={style.page}>
      <TokenList tokens={result} />
    </div>
  )
}

export default UserProfilePage
