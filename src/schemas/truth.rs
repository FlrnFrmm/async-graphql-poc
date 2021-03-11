type User {
    avatarUrl: string
    name: string
    email: string
    walletAddress: string
    bio: string
    mode: Mode
    rememberMe: Boolean
    newsletter: Boolean
  }
  
  type Mode = 'light' | 'dark'
  
  type Artwork {
    tokenId: string
    owner: User
    creator: User
    assetUrl: string
    name: string
    auction: Auction
    countdown: number | null
    description: string
    history: Transaction[]
  }
  
  type Transaction {
    id: string
    bider: User
    datetime: number
    iotaValue: number
    winner: boolean
  }
  
  type Auction {
    countdown: number
    startDate: string
    transactions: Transaction[]
  }