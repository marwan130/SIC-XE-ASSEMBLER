# Frontend Documentation

React TypeScript web application for the SIC-XE Assembler project.

## Overview

The frontend is built with React 18 and features:
- JWT-based authentication
- OAuth integration (Google & GitHub)
- Real-time assembly code editing
- Assembly history management
- Theme system (Neon, Cyber, Pink)
- Custom pixel cursor
- CRT overlay effects

## Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool
- **Tailwind CSS** - Styling
- **Custom Hooks** - State management
- **React Router** - Navigation (if needed)

## Setup

### Prerequisites

- Node.js 18 or later
- npm or yarn

### Installation

1. Navigate to the frontend directory:
```bash
cd assembler-ui
```

2. Install dependencies:
```bash
npm install
```

3. Create environment file (`.env`):
```env
VITE_API_URL=http://localhost:8080
```

4. Start development server:
```bash
npm run dev
```

The frontend will start on `http://localhost:5173`

### Production Build

```bash
npm run build
npm run preview
```

## Project Structure

```
assembler-ui/
├── src/
│   ├── components/
│   │   ├── auth/              # Authentication components
│   │   │   ├── LoginPage.tsx
│   │   │   └── RegisterPage.tsx
│   │   ├── assembler/         # Assembly components
│   │   │   ├── CodeEditor.tsx
│   │   │   ├── OutputPanel.tsx
│   │   │   └── OutputTabs.tsx
│   │   ├── history/           # History components
│   │   │   ├── HistoryPanel.tsx
│   │   │   └── HistoryItem.tsx
│   │   ├── layout/            # Layout components
│   │   │   ├── LeftNavbar.tsx
│   │   │   └── PixelDivider.tsx
│   │   ├── background/        # Background effects
│   │   │   ├── StarField.tsx
│   │   │   └── CRTOverlay.tsx
│   │   ├── cursor/            # Cursor components
│   │   │   └── PixelCursor.tsx
│   │   └── profile/           # Profile components
│   │       └── ProfilePage.tsx
│   ├── hooks/
│   │   ├── useAuth.ts         # Authentication hook
│   │   └── useAssembler.ts    # Assembly hook
│   ├── lib/
│   │   ├── api.ts             # API client
│   │   └── auth.ts            # Auth utilities
│   ├── App.tsx                # Main app component
│   ├── main.tsx               # Entry point
│   └── index.css              # Global styles
├── index.html                 # HTML template
├── package.json               # Dependencies
└── vite.config.ts             # Vite configuration
```

## Components

### Authentication Components

#### LoginPage
Handles user login with email/password and OAuth buttons.

**Features:**
- Email validation (regex)
- Password validation
- OAuth redirect (Google, GitHub)
- Error handling with user-friendly messages

#### RegisterPage
Handles user registration with validation.

**Features:**
- Name validation (min 2 characters)
- Email validation (regex)
- Password validation (4-12 chars, letters + numbers)
- Password confirmation matching
- OAuth redirect (Google, GitHub)

### Assembly Components

#### CodeEditor
Code editor for SIC/XE assembly code.

**Features:**
- Line numbers
- Error display

#### OutputPanel
Displays assembly output with tabbed interface.

**Features:**
- Multiple output tabs (Pass1, Symbol Table, Literal Table, Object Program)
- Copy to clipboard functionality
- Error display

### History Components

#### HistoryPanel
Displays user's assembly history.

**Features:**
- List of saved assemblies
- Delete individual items
- Delete all items
- Load saved assembly

### Layout Components

#### LeftNavbar
Navigation sidebar.

**Features:**
- Navigation between pages
- Collapsible
- Auth status indicator

#### Divider
Paenl divider component.

### Background Components

#### StarField
Animated starfield background effect.

**Features:**
- Twinkling stars
- Smooth animation
- Performance optimized

#### CRTOverlay
CRT scanline overlay effect.

**Features:**
- Scanline pattern
- Flicker animation
- Retro aesthetic

## Custom Hooks

### useAuth
Manages authentication state and operations.

**Methods:**
- `login(email, password)` - Login user
- `register(email, password, name)` - Register user
- `logout()` - Logout user
- `clearError()` - Clear error state

**State:**
- `user` - Current user object
- `isLoggedIn` - Authentication status
- `error` - Error message
- `isInitialized` - Initialization status

### useAssembler
Manages assembly operations and history.

**Methods:**
- `assemble()` - Assemble current code
- `saveSession(title)` - Save assembly session
- `loadSession(session)` - Load saved session
- `deleteSession(id)` - Delete session
- `deleteAllSessions()` - Delete all sessions
- `refreshHistory()` - Refresh history from storage

**State:**
- `code` - Current assembly code
- `outputs` - Assembly output
- `loading` - Loading state
- `history` - Assembly history

## API Client

The API client (`lib/api.ts`) provides methods for interacting with the backend:

```typescript
import { api } from './lib/api';

// Authentication
await api.post('/auth/login', { email, password });
await api.post('/auth/register', { email, password, name });

// Assembly
await api.post('/assemble', { code, title });
await api.get('/history');
await api.delete(`/history/${id}`);
```

## Styling

### Design System

The frontend uses a retro-arcade design:

**Colors:**
- Background: `#0A0A0A`, `#121212`
- Neon Green: `#00FF66`
- Cyber Yellow: `#FFF500`
- Hot Pink: `#FF007A`
- Electric Blue: `#00E0FF`

**Typography:**
- Headings: 18px
- Body text: 12px
- Helper text: 10px
- Font: "JetBrains Mono" (monospace)

**Borders:**
- 2-3px solid black
- Box shadow: `4px 4px 0px #000000`

### Tailwind Configuration

Custom utilities are defined in `index.css`:

```css
.neo-box        /* Standard neo-brutalist box */
.neo-box-pink   /* Pink accent box */
.neo-box-green  /* Green accent box */
.neo-box-cyber  /* Yellow accent box */
.neo-box-blue   /* Blue accent box */
.neo-button     /* Neo-brutalist button */
.font-press     /* Pixel font */
.font-mono-custom /* Monospace font */
```

### Theme System

Themes are applied via CSS variables:

```typescript
const themes = {
  neon: { accent: '#00FF66' },
  cyber: { accent: '#FFF500' },
  pink: { accent: '#FF007A' }
};
```

## Security

### Client-Side Validation
- Email format validation (regex)
- Password strength validation
- Input sanitization
- XSS prevention via React's built-in escaping

### Authentication
- JWT token storage in localStorage
- Token inclusion in API requests
- Automatic token refresh (if implemented)
- Logout functionality

### OAuth
- Secure redirect handling
- Token extraction from URL fragment
- State validation (if implemented)

### API Security
- CORS configuration
- HTTPS in production
- Secure token storage

## Development

### Running in Development

```bash
npm run dev
```

### Building for Production

```bash
npm run build
```

### Preview Production Build

```bash
npm run preview
```

### Linting

```bash
npm run lint
```

### Type Checking

```bash
npm run type-check
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `VITE_API_URL` | Yes | Backend API URL |