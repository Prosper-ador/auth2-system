/* eslint-disable @typescript-eslint/no-explicit-any */
import { useEffect, useState } from 'react';
import { ProtectedApi, Configuration } from '../../../ts-client/api';
import { useNavigate } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { User as UserIcon, Mail, ShieldCheck, LogOut, Clock } from "lucide-react";
import { Label } from '@/components/ui/label';

const api = new ProtectedApi(new Configuration({ basePath: 'http://localhost:3000' }));

const Profile = () => {
  const [user, setUser] = useState<any>(null);
  const [error, setError] = useState('');
  const navigate = useNavigate();
  const [sessionProgress, setSessionProgress] = useState(100);

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (!token) {
      navigate('/login');
      return;
    }
    api.adminRoute({ headers: { Authorization: `Bearer ${token}` } })
      .then(({ data }) => setUser(data))
      .catch(() => {
        setError('Failed to load profile.');
        localStorage.removeItem('token');
        navigate('/login');
      });
  }, [navigate]);

  useEffect(() => {
    const interval = setInterval(() => {
      const tokenExpiry = localStorage.getItem('token_expiry');
      if (tokenExpiry) {
        const now = Date.now();
        const expiry = parseInt(tokenExpiry);
        const remaining = Math.max(0, expiry - now);
        const total = 10 * 60 * 1000; // 10 minutes in ms
        const progress = (remaining / total) * 100;
        setSessionProgress(progress);
      }
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  if (error) return <div className="flex items-center justify-center min-h-screen text-red-500">{error}</div>;
  if (!user) return <div className="flex items-center justify-center min-h-screen">Loading...</div>;

  const statusColor = sessionProgress > 50 ? "bg-emerald-500" : sessionProgress > 20 ? "bg-yellow-500" : "bg-red-500";
  const statusText = sessionProgress > 50 ? "Active" : sessionProgress > 20 ? "Warning" : "Expiring";

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-[#f6f3ff] py-8">
      <div className="flex flex-col items-center mb-8">
        <div className="flex items-center justify-center h-16 w-16 rounded-full bg-gradient-to-br from-purple-400 to-blue-400 mb-4">
          <ShieldCheck className="h-9 w-9 text-white" />
        </div>
        <h1 className="text-3xl font-extrabold text-purple-700 mb-2">Welcome, {user.first_name || user.email}!</h1>
        <p className="text-gray-500 text-lg">Manage your account and session</p>
      </div>
      {/* Session Status Card */}
      <div className="w-full max-w-xl mb-6">
        <Card className="rounded-xl shadow border-0">
          <CardHeader className="flex flex-row items-center gap-2 pb-2">
            <Clock className="h-5 w-5 text-purple-500" />
            <CardTitle className="text-lg font-bold text-purple-700">Session Status</CardTitle>
          </CardHeader>
          <CardContent className="pt-0">
            <p className="text-gray-600 text-sm mb-2">Your session will expire automatically for security</p>
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-700">Session Progress</span>
              <span className={`text-xs font-semibold px-2 py-1 rounded-full text-white ${statusColor}`}>{statusText}</span>
            </div>
            <Progress value={sessionProgress} className="h-2 bg-gray-200" />
            <p className="text-xs text-gray-600 mt-2">Session expires in approximately {Math.ceil((sessionProgress / 100) * 10)} minutes</p>
          </CardContent>
        </Card>
      </div>
      {/* Profile Info Card */}
      <div className="w-full max-w-xl mb-6">
        <Card className="rounded-xl shadow border-0">
          <CardHeader className="flex flex-row items-center gap-2 pb-2">
            <UserIcon className="h-5 w-5 text-purple-500" />
            <CardTitle className="text-lg font-bold text-purple-700">Profile Information</CardTitle>
          </CardHeader>
          <CardContent className="pt-0">
            <p className="text-gray-600 text-sm mb-4">Your account details and information</p>
            <div className="mb-4">
              <Label className="text-xs font-medium text-gray-700 flex items-center gap-2"><UserIcon className="h-4 w-4 text-purple-400" />Full Name</Label>
              <input type="text" value={user.first_name + ' ' + user.last_name} readOnly className="w-full mt-1 p-2 rounded bg-purple-50 border border-purple-100 text-gray-800 font-medium outline-none" />
            </div>
            <div className="mb-4">
              <Label className="text-xs font-medium text-gray-700 flex items-center gap-2"><Mail className="h-4 w-4 text-purple-400" />Email Address</Label>
              <input type="text" value={user.email} readOnly className="w-full mt-1 p-2 rounded bg-purple-50 border border-purple-100 text-gray-800 font-medium outline-none" />
            </div>
            <div className="mb-4">
              <Label className="text-xs font-medium text-gray-700 flex items-center gap-2"><ShieldCheck className="h-4 w-4 text-purple-400" />User ID</Label>
              <input type="text" value={user.id} readOnly className="w-full mt-1 p-2 rounded bg-purple-50 border border-purple-100 text-gray-800 font-medium outline-none" />
            </div>
            <Button
              className="w-full mt-4 py-2 font-semibold text-white bg-gradient-to-r from-pink-500 to-red-500 hover:from-red-500 hover:to-pink-500 transition-all duration-200 shadow-md flex items-center justify-center gap-2"
              onClick={() => {
                localStorage.removeItem('token');
                localStorage.removeItem('token_expiry');
                navigate('/login');
              }}
              aria-label="Sign out securely"
            >
              <LogOut className="inline h-5 w-5 mr-1" /> Sign Out Securely
            </Button>
          </CardContent>
        </Card>
      </div>
      {/* Animations */}
      <style>{`
        @keyframes blob1 { 0%, 100% { transform: scale(1) translateY(0); } 50% { transform: scale(1.1) translateY(20px); } }
        @keyframes blob2 { 0%, 100% { transform: scale(1) translateY(0); } 50% { transform: scale(1.05) translateY(-20px); } }
        .animate-blob1 { animation: blob1 8s ease-in-out infinite; }
        .animate-blob2 { animation: blob2 10s ease-in-out infinite; }
        @keyframes fadeInUp { 0% { opacity: 0; transform: translateY(40px) scale(0.98);} 100% { opacity: 1; transform: translateY(0) scale(1);} }
        .animate-fade-in-up { animation: fadeInUp 0.8s cubic-bezier(.4,0,.2,1) both; }
      `}</style>
    </div>
  );
};

export default Profile;

