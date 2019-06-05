import React, { useCallback } from "react"
import style from "./styles.scss"
import { timeago } from "~/utils/timeago"
import { RemoteData } from "~/utils/remote-data"
import { AccessTokenView } from "~/api"
import check from "~/img/check.svg"

export type TokenRowProps = {
  token: AccessTokenView
  onDeleteToken: (token_id: number) => void
}

const TokenRow: React.FunctionComponent<TokenRowProps> = props => {
  return (
    <tr>
      <td>{props.token.token_partial}</td>
      <td>{timeago(new Date(props.token.created_at)) + " ago"}</td>
      <td>
        <button
          className="button is-black"
          onClick={() => props.onDeleteToken(props.token.id)}
        >
          Delete
        </button>
      </td>
    </tr>
  )
}

export type TokenListProps = {
  tokens: RemoteData<AccessTokenView[]>
  onDeleteToken: (token_id: number) => void
}

export const TokenList: React.FunctionComponent<TokenListProps> = props => {
  return (
    <table className={style["token-table"]}>
      <thead>
        <tr>
          <th className={style["th-title"]}>Token</th>
          <th className={style["th-title"]}>Created</th>
          <th />
        </tr>
      </thead>
      <tbody>
        {props.tokens.type === "Ready" ? (
          props.tokens.data.length === 0 ? (
            <tr>
              <td className={style["td-info"]} colSpan={3}>
                You have no avaliable token
              </td>
            </tr>
          ) : (
            props.tokens.data
              .sort((a, b) => a.id - b.id)
              .map((token, idx) => (
                <TokenRow
                  key={idx}
                  token={token}
                  onDeleteToken={props.onDeleteToken}
                />
              ))
          )
        ) : (
          <tr>
            <td className={style["td-info"]} colSpan={3}>
              Loading
            </td>
          </tr>
        )}
      </tbody>
    </table>
  )
}

export type TokenDisplayProps = {
  token: string
}

export const TokenDisplay: React.FunctionComponent<
  TokenDisplayProps
> = props => {
  return (
    <div className={style["token-display"]}>
      <div className={style["token-display__top"]}>
        <img src={check} />
        <div className={style["token-display__text"]}>
          <p>Token created</p>
          <p>It will never be displayed again</p>
        </div>
      </div>
      <div className={style["token-display__bottom"]}>{props.token}</div>
    </div>
  )
}
