import React, { useCallback } from "react"
import style from "./styles.scss"
import { RemoteData } from "~/utils/remote-data"

export type User = {
  id: number
  name: string
  email: string
  avatar: string
}

type ProfileProps = {
  user: User
}

export const UserProfile: React.FunctionComponent<ProfileProps> = props => {
  return (
    <div className={style["profile"]}>
      <img className={style["avatar"]} src={props.user.avatar} />
      <div className={style["text"]}>
        <div className={style["name"]}>{props.user.name}</div>
        <div className={style["email"]}>{props.user.email}</div>
      </div>
    </div>
  )
}
