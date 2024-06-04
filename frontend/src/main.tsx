import React from 'react'
import ReactDOM from 'react-dom/client'
import { Navigate, Outlet, RouterProvider, createBrowserRouter } from 'react-router-dom'
import './index.css'
import AuthLayout from './auth/AuthLayout.tsx'
import SignUp from './auth/SignUp.tsx'
import SignIn from './auth/SignIn.tsx'
import Products from './Products.tsx'

const PrivateRoutes = () => {
  return localStorage.getItem("jwt") ? <Outlet /> : <Navigate to="/auth" />
}

const router = createBrowserRouter([
  {
    path: "/",
    element: <PrivateRoutes />,
    children: [
      {
        path: "/",
        element: <Products />
      }
    ]
  },
  {
    path: "/auth",
    element: <AuthLayout />,
    children: [
      {
        path: "/auth",
        element: <SignIn />
      },
      {
        path: "/auth/register",
        element: <SignUp />
      }
    ]
  }
])

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
