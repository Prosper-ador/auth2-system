import { Link, useLocation } from 'react-router-dom';
import { useAuth } from '@/hooks/useAuth';
import { useTheme } from '@/hooks/useThemeContext';
import { Sun, Moon, User, Shield } from 'lucide-react';

const Navbar = () => {
  const { user } = useAuth();
  const { theme, toggleTheme } = useTheme();
  const location = useLocation();

  return (
    <nav className="w-full flex items-center justify-between px-6 py-3 bg-white dark:bg-gray-900 shadow-md fixed top-0 left-0 z-50 animate-fade-in-up">
      <div className="flex items-center gap-6">
        <Link to="/profile" className={`font-bold text-lg flex items-center gap-2 ${location.pathname === '/profile' ? 'text-purple-600 dark:text-purple-400' : 'text-gray-700 dark:text-gray-200'} transition-colors`}>
          <User className="h-5 w-5" /> Profile
        </Link>
        {user?.role === 'Admin' && (
          <Link to="/admin/dashboard" className={`font-bold text-lg flex items-center gap-2 ${location.pathname === '/admin/dashboard' ? 'text-emerald-600 dark:text-emerald-400' : 'text-gray-700 dark:text-gray-200'} transition-colors`}>
            <Shield className="h-5 w-5" /> Admin Dashboard
          </Link>
        )}
      </div>
      <button
        onClick={toggleTheme}
        className="rounded-full p-2 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors shadow"
        aria-label="Toggle dark mode"
      >
        {theme === 'dark' ? <Sun className="h-5 w-5 text-yellow-400 animate-spin-slow" /> : <Moon className="h-5 w-5 text-gray-700 dark:text-gray-200" />}
      </button>
      <style>{`
        @keyframes fadeInUp { 0% { opacity: 0; transform: translateY(-20px);} 100% { opacity: 1; transform: translateY(0);} }
        .animate-fade-in-up { animation: fadeInUp 0.7s cubic-bezier(.4,0,.2,1) both; }
        .animate-spin-slow { animation: spin 2s linear infinite; }
      `}</style>
    </nav>
  );
};

export default Navbar;
