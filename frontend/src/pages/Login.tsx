/* eslint-disable @typescript-eslint/no-explicit-any */
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useToast } from "@/hooks/use-toast";
import { Mail, Lock, LogIn, Sparkles } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";
import { LoginRequest } from '../../ts-client/api';

const Login = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const { toast } = useToast();
  const { login } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      const credentials: LoginRequest = {
        email,
        password,
      };
      
      await login(credentials);
      // Navigation is handled by the useAuth hook
    } catch (err: any) {
      toast({ 
        title: "Login failed", 
        description: err?.response?.data?.error || 'Login failed', 
        variant: "destructive" 
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="relative min-h-screen flex items-center justify-center bg-gradient-to-b from-[#f5f7fa] to-[#c3cfe2] dark:from-gray-900 dark:to-gray-800 transition-colors duration-300">
      <div className="pointer-events-none absolute -top-40 -right-40 w-96 h-96 bg-gradient-to-br from-purple-400 to-blue-500 dark:from-purple-700 dark:to-blue-800 rounded-full opacity-20 animate-blob1"></div>
      <div className="pointer-events-none absolute -bottom-40 -left-40 w-96 h-96 bg-gradient-to-tr from-blue-400 to-indigo-500 dark:from-blue-900 dark:to-indigo-900 rounded-full opacity-20 animate-blob2"></div>
      <Card className="w-full max-w-md rounded-2xl shadow-lg border-0 p-0 animate-fade-in-up bg-white dark:bg-gray-900 transition-colors duration-300">
        <CardHeader className="flex flex-col items-center gap-2 bg-transparent pt-8 pb-2">
          <div className="flex items-center justify-center h-14 w-14 rounded-full bg-gradient-to-br from-purple-400 to-blue-400 dark:from-purple-700 dark:to-blue-800 mb-2 animate-float">
            <Sparkles className="h-8 w-8 text-white" />
          </div>
          <CardTitle className="text-2xl font-extrabold text-purple-700 dark:text-purple-300">Welcome Back</CardTitle>
          <p className="text-gray-500 dark:text-gray-300 text-sm">Sign in to your account to continue</p>
        </CardHeader>
        <CardContent className="pt-2 pb-6 px-8">
          <form className="space-y-5" onSubmit={handleSubmit} autoComplete="on" aria-label="Login form">
            <div>
              <Label htmlFor="email" className="font-semibold dark:text-gray-200">Email</Label>
              <div className="relative mt-1">
                <Mail className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
                <Input
                  id="email"
                  type="email"
                  value={email}
                  onChange={e => setEmail(e.target.value)}
                  placeholder="user@example.com"
                  required
                  autoFocus
                  className="pl-10 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 focus:border-purple-400 focus:ring-2 focus:ring-purple-100 dark:focus:border-purple-600 dark:focus:ring-purple-900 text-gray-900 dark:text-gray-100"
                  aria-label="Email address"
                />
              </div>
            </div>
            <div>
              <Label htmlFor="password" className="font-semibold dark:text-gray-200">Password</Label>
              <div className="relative mt-1">
                <Lock className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
                <Input
                  id="password"
                  type="password"
                  value={password}
                  onChange={e => setPassword(e.target.value)}
                  placeholder="Enter your password"
                  required
                  className="pl-10 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 focus:border-purple-400 focus:ring-2 focus:ring-purple-100 dark:focus:border-purple-600 dark:focus:ring-purple-900 text-gray-900 dark:text-gray-100"
                  aria-label="Password"
                />
              </div>
            </div>
            <Button
              type="submit"
              className="w-full py-2 font-semibold text-white bg-gradient-to-r from-blue-500 to-purple-500 hover:from-purple-500 hover:to-blue-500 transition-all duration-200 shadow-md flex items-center justify-center gap-2"
              disabled={loading}
              aria-busy={loading}
              aria-label="Sign in"
            >
              <span className="inline-block"><Lock className="inline h-5 w-5 mr-1" /></span>
              {loading ? "Signing in..." : "Sign In"}
            </Button>
          </form>
          <div className="mt-6 text-center text-sm text-gray-500 dark:text-gray-300">
            Don&apos;t have an account?{' '}
            <a href="/register" className="text-purple-600 dark:text-purple-400 font-semibold hover:underline">Create one now</a>
          </div>
          <div className="mt-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-3 text-xs text-gray-600 dark:text-gray-300 text-center">
            Demo: use <span className="font-mono bg-gray-100 dark:bg-gray-900 px-1 rounded text-blue-700 dark:text-blue-300">user@example.com</span> / <span className="font-mono bg-gray-100 dark:bg-gray-900 px-1 rounded text-blue-700 dark:text-blue-300">password</span>
          </div>
        </CardContent>
      </Card>
      {/* Animations */}
      <style>{`
        @keyframes blob1 {
          0%, 100% { transform: scale(1) translateY(0); }
          50% { transform: scale(1.1) translateY(20px); }
        }
        @keyframes blob2 {
          0%, 100% { transform: scale(1) translateY(0); }
          50% { transform: scale(1.05) translateY(-20px); }
        }
        .animate-blob1 { animation: blob1 8s ease-in-out infinite; }
        .animate-blob2 { animation: blob2 10s ease-in-out infinite; }
        @keyframes fadeInUp {
          0% { opacity: 0; transform: translateY(40px) scale(0.98);}
          100% { opacity: 1; transform: translateY(0) scale(1);}
        }
        .animate-fade-in-up { animation: fadeInUp 0.8s cubic-bezier(.4,0,.2,1) both; }
        @keyframes float {
          0%, 100% { transform: translateY(0);}
          50% { transform: translateY(-10px);}
        }
        .animate-float { animation: float 2.5s ease-in-out infinite; }
      `}</style>
    </div>
  );
};

export default Login;
