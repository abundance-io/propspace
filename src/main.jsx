import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.jsx";
import "./index.css";
import { createBrowserRouter } from "react-router-dom";
import { Landing } from "./pages/landing/landing.jsx";
import { Dashboard } from "./pages/dashboard/dashboard.jsx";
import { Error } from "./pages/error/error.jsx";
import { RouterProvider } from "react-router";
import { WebsiteLayout } from "./layouts/website/websiteLayout.jsx";
import { AllBlogs } from "./pages/blog/blog.jsx";
import { BlogEntry } from "./pages/blog/entry.jsx";
import { Login } from "./pages/auth/login.jsx";
import { Register } from "./pages/auth/register.jsx";
import { AppLayout } from "./layouts/app/app.jsx";
import { AuthLayout } from "./layouts/app/auth/auth.jsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    errorElement: <Error />,
    children: [
      {
        element: <WebsiteLayout />,
        children: [
          {
            index: true,
            element: <Landing />,
          },
          {
            path: "blog",
            element: <AllBlogs />,
          },
          {
            path: "blog/:title",
            element: <BlogEntry />,
          },
        ],
      },
      {
        path: "app",
        element: <AppLayout />,
        children: [
          {
            index: true,
            path: "dashboard",
            element: <Dashboard />,
          },
        ],
      },
      {
        path: "/",
        element: <AuthLayout />,
        children: [
          {
            path: "login",
            element: <Login />,
          },
          {
            path: "register",
            element: <Register />,
          },
        ],
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
