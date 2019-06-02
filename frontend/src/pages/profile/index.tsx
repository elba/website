import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { TokenList, TokenProps } from "~components/token-list"
import { RemoteData } from "~/utils/remote-data"

const testResults: TokenProps[] = new Array(5)
  .fill({
    token_partial: "3423......2344",
    created_at: new Date(),
  })
  .map((item, idx) => ({ ...item, token_id: idx }))

export const UserProfilePage: React.FunctionComponent = () => {
  const [result, setResult] = useState<RemoteData<TokenProps[]>>({
    type: "Not Asked",
  })
  useEffect(() => {
    if (result.type === "Not Asked") {
      setResult({ type: "Started" })
      setTimeout(() => {
        setResult({ type: "Done", data: testResults })
      }, 1000)
    }
  })
  return (
    <div className={style.page}>
      <div className={style["token-section"]}>
        <div className={style["token-section__title"]}>
          <h2>Access Tokens</h2>
          <button className="button is-purple">Create Token</button>
        </div>
        <TokenList tokens={result} />
      </div>
    </div>
  )
}

export default UserProfilePage
