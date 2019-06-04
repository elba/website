import React, { useState, useEffect, useCallback } from "react"
import style from "./styles.scss"
import { GlobalStateConsumer } from "~/utils/global-state.tsx"
import { TokenList, TokenDisplay } from "~components/token-list"
import { UserProfile } from "~components/user-profile"
import { RemoteData } from "~/utils/remote-data"
import {
  AccessTokenView,
  list_tokens,
  logout,
  create_token,
  remove_token,
} from "~/api"
import { Redirect } from "react-router"
import { Navbar } from "~components/navbar"

export const UserProfilePage: React.FunctionComponent = () => {
  const [tokenDisplay, setTokenDisplay] = useState<string | void>(undefined)
  const [tokens, setTokens] = useState<RemoteData<AccessTokenView[]>>({
    type: "Not Asked",
  })

  useEffect(() => {
    if (tokens.type === "Not Asked") {
      loadTokens()
    }
  })

  const loadTokens = async () => {
    setTokens({ type: "Started" })
    let tokens = await list_tokens()
    setTokens({ type: "Done", data: tokens })
  }

  const onCreateToken = async () => {
    let token = await create_token()
    setTokenDisplay(token.token)
    loadTokens()
  }

  const onDeleteToken = async (token_id: number) => {
    if (confirm("Are you sure you want to delete this token?")) {
      await remove_token(token_id)
      setTokenDisplay(undefined)
      loadTokens()
    }
  }

  return (
    <div>
      <Navbar />
      <GlobalStateConsumer>
        {globalState =>
          globalState.user !== undefined ? (
            <div className={style.page}>
              <div className={style["profile-section"]}>
                <h2>Profile</h2>
                <div className={style["profile-section__content"]}>
                  <UserProfile user={globalState.user} />
                </div>
              </div>
              <div className={style["token-section"]}>
                <div className={style["token-section__title"]}>
                  <h2>Access Tokens</h2>
                  <button className="button is-purple" onClick={onCreateToken}>
                    Create Token
                  </button>
                </div>
                <TokenList tokens={tokens} onDeleteToken={onDeleteToken} />
                {tokenDisplay !== undefined ? (
                  <div className={style["token-display"]}>
                    <TokenDisplay token={tokenDisplay} />
                  </div>
                ) : (
                  undefined
                )}
              </div>
              <div className={style["logout"]}>
                <button
                  className="button is-purple"
                  onClick={() => onLogout(globalState.fetchUser)}
                >
                  Log Out
                </button>
              </div>
            </div>
          ) : (
            <Redirect to="/" />
          )
        }
      </GlobalStateConsumer>
    </div>
  )
}

async function onLogout(fetchUser: () => void) {
  await logout()
  fetchUser()
}

export default UserProfilePage
