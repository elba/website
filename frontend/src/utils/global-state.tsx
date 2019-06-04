import React, { createContext, useReducer, useState, useEffect } from "react"
import { UserView, show_user_self } from "~/api"

type GlobalState = {
  searchQuery: string
  setSearchQuery: (query: string) => void
  user?: UserView
  fetchUser: () => void
}

const { Provider, Consumer } = createContext<GlobalState>({
  searchQuery: "",
  setSearchQuery: () => undefined,
  user: undefined,
  fetchUser: () => undefined,
})

export const GlobalStateProvider: React.FunctionComponent = props => {
  const [searchQuery, setSearchQuery] = useState<string>("")
  const [user, setUser] = useState<UserView | undefined>(undefined)

  useEffect(() => {
    fetchUser()
  }, [props])

  const fetchUser = () => {
    async function load() {
      let user = await show_user_self()
      if (user == null) {
        setUser(undefined)
      } else {
        setUser(user)
      }
    }
    load()
  }

  return (
    <Provider
      value={{
        searchQuery: searchQuery,
        setSearchQuery: setSearchQuery,
        user: user,
        fetchUser: fetchUser,
      }}
    >
      {props.children}
    </Provider>
  )
}

export const GlobalStateConsumer = Consumer
