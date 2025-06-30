/* eslint-disable @typescript-eslint/no-explicit-any */
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useToast } from "@/hooks/use-toast";
import { Mail, Lock, User, Sparkles } from "lucide-react";
import { useAuth } from "@/hooks/useAuth";
import { RegisterRequest } from '../../../ts-client/api';

const Register = () => {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();
  const { toast } = useToast();
  const { register } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (password !== confirmPassword) {
      toast({ 
        title: "Passwords don't match", 
        description: "Please make sure your passwords match", 
        variant: "destructive" 
      });
      return;
    }

    setLoading(true);
    try {
      const userData: RegisterRequest = {
        first_name: firstName,
        last_name: lastName,
        email,
        password,
        confirm_password: confirmPassword,
      };
      
      await register(userData);
      // Navigation is handled by the useAuth hook
    } catch (err: any) {
      toast({ 
        title: "Registration failed", 
        description: err?.response?.data?.error || 'Registration failed', 
        variant: "destructive" 
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-b from-[#f5faf7] to-[#d2f1e6] dark:from-gray-900 dark:to-gray-800 transition-colors duration-300">
      <Card className="w-full max-w-md rounded-2xl shadow-lg border-0 p-0 bg-white dark:bg-gray-900 animate-fade-in-up transition-colors duration-300">
        <CardHeader className="flex flex-col items-center gap-2 bg-transparent pt-8 pb-2">
          <div className="flex items-center justify-center h-14 w-14 rounded-full bg-gradient-to-br from-green-400 to-emerald-400 dark:from-green-700 dark:to-emerald-800 mb-2 animate-float">
            <Sparkles className="h-8 w-8 text-white" />
          </div>
          <CardTitle className="text-2xl font-extrabold text-emerald-700 dark:text-emerald-300">Create Account</CardTitle>
          <p className="text-gray-500 dark:text-gray-300 text-sm">Join us today and start your journey</p>
        </CardHeader>
        <CardContent>
          <form className="space-y-6" onSubmit={handleSubmit} autoComplete="on" aria-label="Register form">
            <div className="space-y-2">
              <Label htmlFor="first_name" className="dark:text-gray-200">First Name</Label>
              <Input
                id="first_name"
                type="text"
                value={firstName}
                onChange={e => setFirstName(e.target.value)}
                placeholder="First name"
                required
                autoFocus
                aria-label="First name"
                className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-900 dark:text-gray-100"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="last_name" className="dark:text-gray-200">Last Name</Label>
              <Input
                id="last_name"
                type="text"
                value={lastName}
                onChange={e => setLastName(e.target.value)}
                placeholder="Last name"
                required
                aria-label="Last name"
                className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-900 dark:text-gray-100"
              />
            </div>
            <div>
              <Label htmlFor="email" className="font-semibold dark:text-gray-200">Email</Label>
              <div className="relative mt-1">
                <Mail className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
                <Input
                  id="email"
                  type="email"
                  value={email}
                  onChange={e => setEmail(e.target.value)}
                  placeholder="you@example.com"
                  required
                  className="pl-10 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 focus:border-emerald-400 focus:ring-2 focus:ring-emerald-100 dark:focus:border-emerald-600 dark:focus:ring-emerald-900 text-gray-900 dark:text-gray-100"
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
                  placeholder="Create a secure password"
                  required
                  className="pl-10 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 focus:border-emerald-400 focus:ring-2 focus:ring-emerald-100 dark:focus:border-emerald-600 dark:focus:ring-emerald-900 text-gray-900 dark:text-gray-100"
                  aria-label="Password"
                />
              </div>
            </div>
            <div className="space-y-2">
              <Label htmlFor="confirm_password" className="dark:text-gray-200">Confirm Password</Label>
              <Input
                id="confirm_password"
                type="password"
                value={confirmPassword}
                onChange={e => setConfirmPassword(e.target.value)}
                placeholder="Confirm your password"
                required
                aria-label="Confirm password"
                className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-900 dark:text-gray-100"
              />
            </div>
            <Button
              type="submit"
              className="w-full py-2 font-semibold text-white bg-gradient-to-r from-emerald-500 to-green-500 hover:from-green-500 hover:to-emerald-500 transition-all duration-200 shadow-md flex items-center justify-center gap-2"
              disabled={loading}
              aria-busy={loading}
              aria-label="Create account"
            >
              <span className="inline-block"><User className="inline h-5 w-5 mr-1" /></span>
              {loading ? "Creating..." : "Create Account"}
            </Button>
          </form>
          <div className="mt-6 text-center text-sm text-gray-500 dark:text-gray-300">
            Already have an account?{' '}
            <a href="/login" className="text-emerald-600 dark:text-emerald-400 font-semibold hover:underline">Sign in here</a>
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

export default Register;
