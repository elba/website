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
    <div>
      <div>
        <img src={props.user.avatar} />
      </div>
      <div>{props.user.name}</div>
      <div>{props.user.email}</div>
    </div>
  )
}
