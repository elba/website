import React, { useState, useEffect } from "react"
import style from "./styles.scss"
import { TokenList, TokenProps, TokenDisplay } from "~components/token-list"
import { UserProfile, User } from "~components/user-profile"
import { RemoteData } from "~/utils/remote-data"
import avatar from "~/img/avatar.jpg"

const testResults: TokenProps[] = new Array(5)
  .fill({
    token_partial: "3423......2344",
    created_at: new Date(),
  })
  .map((item, idx) => ({ ...item, token_id: idx }))

const testUser: User = {
  id: 0,
  avatar: avatar,
  name: "andylokandy",
  email: "andylokandy@hotmail.com",
}

export const UserProfilePage: React.FunctionComponent = () => {
  const [result, setResult] = useState<RemoteData<TokenProps[]>>({
    type: "Not Asked",
  })
  const [tokenDisplay, setTokenDisplay] = useState<string | null>(null)
  useEffect(() => {
    if (result.type === "Not Asked") {
      setResult({ type: "Started" })
      setTimeout(() => {
        setResult({ type: "Done", data: testResults })
        setTokenDisplay("9qc21ac89c6zv9zz89xv")
      }, 1000)
    }
  })
  return (
    <div className={style.page}>
      <div className={style["profile-section"]}>
        <h2>Profile</h2>
        <div className={style["profile-section__content"]}>
          <UserProfile user={testUser} />
        </div>
      </div>
      <div className={style["token-section"]}>
        <div className={style["token-section__title"]}>
          <h2>Access Tokens</h2>
          <button className="button is-purple">Create Token</button>
        </div>
        {tokenDisplay !== null ? (
          <div className={style["token-display"]}>
            <TokenDisplay token={tokenDisplay} />
          </div>
        ) : (
          <div />
        )}
        <TokenList tokens={result} />
      </div>
      <div className={style["logout"]}>
        <button className="button is-purple">Logout</button>
      </div>
    </div>
  )
}

export default UserProfilePage
