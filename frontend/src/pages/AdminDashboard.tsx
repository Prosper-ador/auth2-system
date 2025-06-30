/* eslint-disable @typescript-eslint/no-explicit-any */
import { useEffect, useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useToast } from '@/hooks/use-toast';
import { useAuth } from '@/hooks/useAuth';
import { RegisterRequest, UserResponse } from '../../ts-client/api';
import { ShieldCheck, UserPlus, Users, Loader2 } from 'lucide-react';

const AdminDashboard = () => {
  const { user } = useAuth();
  const { toast } = useToast();
  const [users, setUsers] = useState<UserResponse[]>([]);
  const [loading, setLoading] = useState(true);
  const [form, setForm] = useState({
    first_name: '',
    last_name: '',
    email: '',
    password: '',
    confirm_password: '',
  });
  const [registering, setRegistering] = useState(false);

  useEffect(() => {
    const fetchUsers = async () => {
      setLoading(true);
      try {
        const token = localStorage.getItem('auth_token');
        const res = await fetch(`${import.meta.env.VITE_API_BASE_URL || 'auth2-system-production.up.railway.app'}/admin/dashboard`, {
          headers: { Authorization: `Bearer ${token}` },
        });
        if (!res.ok) throw new Error('Failed to fetch users');
        const data = await res.json();
        setUsers(data.users || []);
      } catch (err: any) {
        toast({ title: 'Error', description: err.message, variant: 'destructive' });
      } finally {
        setLoading(false);
      }
    };
    if (user?.role === 'Admin') fetchUsers();
  }, [user, toast]);

  const handleInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    setForm({ ...form, [e.target.id]: e.target.value });
  };

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    if (form.password !== form.confirm_password) {
      toast({ title: "Passwords don't match", description: 'Please check your passwords.', variant: 'destructive' });
      return;
    }
    setRegistering(true);
    try {
      const token = localStorage.getItem('auth_token');
      const res = await fetch(`${import.meta.env.VITE_API_BASE_URL || 'auth2-system-production.up.railway.app'}/admin/register`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(form),
      });
      if (!res.ok) {
        const err = await res.json();
        throw new Error(err.error || 'Failed to register admin');
      }
      toast({ title: 'Admin Registered', description: 'A new admin has been created.' });
      setForm({ first_name: '', last_name: '', email: '', password: '', confirm_password: '' });
    } catch (err: any) {
      toast({ title: 'Registration Failed', description: err.message, variant: 'destructive' });
    } finally {
      setRegistering(false);
    }
  };

  if (user?.role !== 'Admin') {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-50 to-purple-50">
        <Card className="p-8 rounded-2xl shadow-xl border-0 animate-fade-in-up">
          <CardHeader className="flex flex-col items-center gap-2">
            <ShieldCheck className="h-10 w-10 text-purple-500" />
            <CardTitle className="text-2xl font-bold text-purple-700">Admin Access Required</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-gray-600">You do not have permission to view this page.</p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-gray-50 to-purple-50 py-8">
      <div className="w-full max-w-4xl mb-8 animate-fade-in-up">
        <Card className="rounded-xl shadow border-0 mb-6">
          <CardHeader className="flex flex-row items-center gap-2 pb-2">
            <Users className="h-6 w-6 text-purple-500" />
            <CardTitle className="text-xl font-bold text-purple-700">User Stats</CardTitle>
          </CardHeader>
          <CardContent className="pt-0">
            {loading ? (
              <div className="flex items-center gap-2 text-gray-500"><Loader2 className="animate-spin" /> Loading users...</div>
            ) : (
              <div className="flex flex-col gap-2">
                <div className="text-lg font-semibold">Total Users: {users.length}</div>
                <div className="overflow-x-auto">
                  <table className="min-w-full text-sm text-left">
                    <thead>
                      <tr className="bg-purple-100">
                        <th className="px-4 py-2">ID</th>
                        <th className="px-4 py-2">Name</th>
                        <th className="px-4 py-2">Email</th>
                        <th className="px-4 py-2">Role</th>
                      </tr>
                    </thead>
                    <tbody>
                      {users.map(u => (
                        <tr key={u.id} className="even:bg-purple-50">
                          <td className="px-4 py-2">{u.id}</td>
                          <td className="px-4 py-2">{u.first_name} {u.last_name}</td>
                          <td className="px-4 py-2">{u.email}</td>
                          <td className="px-4 py-2">{u.role}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
        <Card className="rounded-xl shadow border-0 animate-fade-in-up">
          <CardHeader className="flex flex-row items-center gap-2 pb-2">
            <UserPlus className="h-6 w-6 text-emerald-500" />
            <CardTitle className="text-xl font-bold text-emerald-700">Register New Admin</CardTitle>
          </CardHeader>
          <CardContent className="pt-0">
            <form className="space-y-4" onSubmit={handleRegister} autoComplete="on" aria-label="Register admin form">
              <div className="flex gap-4">
                <div className="flex-1">
                  <Label htmlFor="first_name">First Name</Label>
                  <Input id="first_name" value={form.first_name} onChange={handleInput} required placeholder="First name" />
                </div>
                <div className="flex-1">
                  <Label htmlFor="last_name">Last Name</Label>
                  <Input id="last_name" value={form.last_name} onChange={handleInput} required placeholder="Last name" />
                </div>
              </div>
              <div>
                <Label htmlFor="email">Email</Label>
                <Input id="email" type="email" value={form.email} onChange={handleInput} required placeholder="admin@example.com" />
              </div>
              <div className="flex gap-4">
                <div className="flex-1">
                  <Label htmlFor="password">Password</Label>
                  <Input id="password" type="password" value={form.password} onChange={handleInput} required placeholder="Password" />
                </div>
                <div className="flex-1">
                  <Label htmlFor="confirm_password">Confirm Password</Label>
                  <Input id="confirm_password" type="password" value={form.confirm_password} onChange={handleInput} required placeholder="Confirm password" />
                </div>
              </div>
              <Button type="submit" className="w-full py-2 font-semibold text-white bg-gradient-to-r from-emerald-500 to-green-500 hover:from-green-500 hover:to-emerald-500 transition-all duration-200 shadow-md flex items-center justify-center gap-2" disabled={registering} aria-busy={registering} aria-label="Register admin">
                {registering ? <Loader2 className="animate-spin h-5 w-5 mr-2" /> : <UserPlus className="inline h-5 w-5 mr-1" />} Register Admin
              </Button>
            </form>
          </CardContent>
        </Card>
      </div>
      <style>{`
        @keyframes fadeInUp { 0% { opacity: 0; transform: translateY(40px) scale(0.98);} 100% { opacity: 1; transform: translateY(0) scale(1);} }
        .animate-fade-in-up { animation: fadeInUp 0.8s cubic-bezier(.4,0,.2,1) both; }
      `}</style>
    </div>
  );
};

export default AdminDashboard;
