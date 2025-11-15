'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { Modal, ModalHeader, ModalBody } from '@/components/ui/modal';
import { Input } from '@/components/ui/input';
import { useGlobalSearch } from '@/lib/hooks';
import {
  Search,
  User,
  FileText,
  TestTube,
  ClipboardList,
  CreditCard,
  Settings,
  Home,
  Users,
  Beaker,
  Package,
  Activity,
  DollarSign,
  FileBarChart,
  Microscope,
  Wrench,
  ChevronRight,
  Clock,
} from 'lucide-react';

interface SearchResult {
  id: string;
  type: 'patient' | 'order' | 'sample' | 'result' | 'invoice' | 'navigation';
  title: string;
  subtitle?: string;
  url: string;
  metadata?: string;
}

const navigationItems = [
  { title: 'Dashboard', url: '/dashboard', icon: Home, category: 'Navigation' },
  { title: 'Patients', url: '/dashboard/patients', icon: Users, category: 'Navigation' },
  { title: 'Register Patient', url: '/dashboard/patients/register', icon: User, category: 'Actions' },
  { title: 'Orders', url: '/dashboard/orders', icon: ClipboardList, category: 'Navigation' },
  { title: 'Create Order', url: '/dashboard/orders/create', icon: FileText, category: 'Actions' },
  { title: 'Samples', url: '/dashboard/samples', icon: TestTube, category: 'Navigation' },
  { title: 'Collect Sample', url: '/dashboard/samples/collect', icon: Beaker, category: 'Actions' },
  { title: 'Results', url: '/dashboard/results', icon: FileBarChart, category: 'Navigation' },
  { title: 'Review Results', url: '/dashboard/results/review', icon: Activity, category: 'Actions' },
  { title: 'Reports', url: '/dashboard/reports', icon: FileText, category: 'Navigation' },
  { title: 'Generate Report', url: '/dashboard/reports/generate', icon: FileBarChart, category: 'Actions' },
  { title: 'Quality Control', url: '/dashboard/qc', icon: Microscope, category: 'Navigation' },
  { title: 'Equipment', url: '/dashboard/equipment', icon: Wrench, category: 'Navigation' },
  { title: 'Inventory', url: '/dashboard/inventory', icon: Package, category: 'Navigation' },
  { title: 'Billing', url: '/dashboard/billing', icon: DollarSign, category: 'Navigation' },
  { title: 'Settings', url: '/dashboard/settings', icon: Settings, category: 'Navigation' },
];

export function CommandPalette() {
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [isSearching, setIsSearching] = useState(false);

  const { search } = useGlobalSearch();

  // Handle keyboard shortcut (Cmd+K or Ctrl+K)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setOpen(true);
      }
      if (e.key === 'Escape') {
        setOpen(false);
        setQuery('');
        setResults([]);
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, []);

  // Handle arrow navigation
  // eslint-disable-next-line react-hooks/exhaustive-deps
  useEffect(() => {
    if (!open) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIndex((prev) => (prev + 1) % results.length);
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIndex((prev) => (prev - 1 + results.length) % results.length);
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (results[selectedIndex]) {
          handleSelect(results[selectedIndex]);
        }
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [open, results, selectedIndex]);

  // Perform search
  useEffect(() => {
    if (!open) return; // Only run when modal is open

    if (!query.trim()) {
      // Show navigation items when no query
      const navResults: SearchResult[] = navigationItems.map((item) => ({
        id: item.url,
        type: 'navigation',
        title: item.title,
        subtitle: item.category,
        url: item.url,
      }));
      setResults(navResults);
      setSelectedIndex(0);
      return;
    }

    const performSearch = async () => {
      setIsSearching(true);
      try {
        const searchResults = await search(query);

        // Filter navigation items
        const filteredNav = navigationItems
          .filter((item) =>
            item.title.toLowerCase().includes(query.toLowerCase())
          )
          .map((item) => ({
            id: item.url,
            type: 'navigation' as const,
            title: item.title,
            subtitle: item.category,
            url: item.url,
          }));

        // Combine results
        const combined = [
          ...searchResults.patients.map((p: { id: string; firstName: string; lastName: string; mrn: string; phone: string }) => ({
            id: p.id,
            type: 'patient' as const,
            title: `${p.firstName} ${p.lastName}`,
            subtitle: `Patient • ${p.mrn}`,
            url: `/dashboard/patients/${p.id}`,
            metadata: p.phone,
          })),
          ...searchResults.orders.map((o: { id: string; orderNumber: string; status: string; createdAt: string; patient: { firstName: string; lastName: string } }) => ({
            id: o.id,
            type: 'order' as const,
            title: o.orderNumber,
            subtitle: `Order • ${o.patient.firstName} ${o.patient.lastName}`,
            url: `/dashboard/orders/${o.id}`,
            metadata: new Date(o.createdAt).toLocaleDateString(),
          })),
          ...searchResults.samples.map((s: { id: string; sampleNumber: string; status: string; sampleType: string; order: { patient: { firstName: string; lastName: string } } }) => ({
            id: s.id,
            type: 'sample' as const,
            title: s.sampleNumber,
            subtitle: `Sample • ${s.order.patient.firstName} ${s.order.patient.lastName}`,
            url: `/dashboard/samples/${s.id}`,
            metadata: s.sampleType,
          })),
          ...searchResults.invoices.map((i: { id: string; invoiceNumber: string; status: string; totalAmount: number; order: { patient: { firstName: string; lastName: string } } }) => ({
            id: i.id,
            type: 'invoice' as const,
            title: i.invoiceNumber,
            subtitle: `Invoice • ${i.order.patient.firstName} ${i.order.patient.lastName}`,
            url: `/dashboard/billing/${i.id}`,
            metadata: `₹${i.totalAmount.toLocaleString()}`,
          })),
          ...filteredNav,
        ];

        setResults(combined);
        setSelectedIndex(0);
      } catch (error) {
        console.error('Search error:', error);
        setResults([]);
      } finally {
        setIsSearching(false);
      }
    };

    const debounceTimer = setTimeout(performSearch, 300);
    return () => clearTimeout(debounceTimer);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, query]);

  const handleSelect = (result: SearchResult) => {
    router.push(result.url);
    setOpen(false);
    setQuery('');
    setResults([]);
  };

  const getIcon = (type: string) => {
    switch (type) {
      case 'patient':
        return <User className="h-4 w-4 text-blue-600" />;
      case 'order':
        return <ClipboardList className="h-4 w-4 text-purple-600" />;
      case 'sample':
        return <TestTube className="h-4 w-4 text-green-600" />;
      case 'result':
        return <FileBarChart className="h-4 w-4 text-orange-600" />;
      case 'invoice':
        return <CreditCard className="h-4 w-4 text-yellow-600" />;
      case 'navigation':
        return <ChevronRight className="h-4 w-4 text-gray-400" />;
      default:
        return <Search className="h-4 w-4 text-gray-400" />;
    }
  };

  return (
    <>
      {/* Trigger Button (can be placed in navbar) */}
      <button
        onClick={() => setOpen(true)}
        className="flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-600 transition-colors hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700"
      >
        <Search className="h-4 w-4" />
        <span className="hidden sm:inline">Search...</span>
        <kbd className="hidden rounded border border-gray-300 bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-600 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-400 sm:inline">
          ⌘K
        </kbd>
      </button>

      {/* Command Palette Modal */}
      <Modal open={open} onClose={() => setOpen(false)}>
        <div className="w-full max-w-2xl">
          <ModalHeader>
            <div className="flex items-center gap-3 border-b border-gray-200 pb-4 dark:border-gray-700">
              <Search className="h-5 w-5 text-gray-400" />
              <Input
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search patients, orders, samples, or navigate..."
                className="flex-1 border-0 bg-transparent p-0 text-lg focus:ring-0"
                autoFocus
              />
              {isSearching && (
                <div className="flex items-center gap-2 text-sm text-gray-500">
                  <Clock className="h-4 w-4 animate-spin" />
                  Searching...
                </div>
              )}
            </div>
          </ModalHeader>
          <ModalBody>
            <div className="max-h-96 overflow-y-auto">
              {results.length === 0 ? (
                <div className="py-12 text-center text-gray-500">
                  <Search className="mx-auto mb-2 h-8 w-8 text-gray-400" />
                  <p className="text-sm">
                    {query ? 'No results found' : 'Start typing to search...'}
                  </p>
                  <p className="mt-1 text-xs text-gray-400">
                    Search for patients, orders, samples, invoices, or navigate to pages
                  </p>
                </div>
              ) : (
                <div className="space-y-1">
                  {/* Group results by type */}
                  {['patient', 'order', 'sample', 'invoice', 'navigation'].map((type) => {
                    const typeResults = results.filter((r) => r.type === type);
                    if (typeResults.length === 0) return null;

                    return (
                      <div key={type} className="mb-4">
                        <div className="mb-2 px-2 text-xs font-semibold uppercase tracking-wide text-gray-500">
                          {type === 'navigation' ? 'Pages' : `${type}s`}
                        </div>
                        {typeResults.map(result => {
                          const globalIndex = results.indexOf(result);
                          return (
                            <div
                              key={result.id}
                              onClick={() => handleSelect(result)}
                              onMouseEnter={() => setSelectedIndex(globalIndex)}
                              className={`flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2 transition-colors ${
                                selectedIndex === globalIndex
                                  ? 'bg-blue-50 dark:bg-blue-950/50'
                                  : 'hover:bg-gray-50 dark:hover:bg-gray-800'
                              }`}
                            >
                              <div className="flex-shrink-0">{getIcon(result.type)}</div>
                              <div className="flex-1 overflow-hidden">
                                <p className="truncate font-medium text-gray-900 dark:text-white">
                                  {result.title}
                                </p>
                                <p className="truncate text-sm text-gray-600 dark:text-gray-400">
                                  {result.subtitle}
                                </p>
                              </div>
                              {result.metadata && (
                                <div className="flex-shrink-0 text-xs text-gray-500">
                                  {result.metadata}
                                </div>
                              )}
                              <ChevronRight className="h-4 w-4 flex-shrink-0 text-gray-400" />
                            </div>
                          );
                        })}
                      </div>
                    );
                  })}
                </div>
              )}
            </div>

            {/* Footer with keyboard shortcuts */}
            <div className="mt-4 flex items-center justify-between border-t border-gray-200 pt-3 text-xs text-gray-500 dark:border-gray-700">
              <div className="flex items-center gap-4">
                <div className="flex items-center gap-1">
                  <kbd className="rounded border border-gray-300 bg-gray-100 px-1.5 py-0.5 font-mono dark:border-gray-600 dark:bg-gray-700">
                    ↑
                  </kbd>
                  <kbd className="rounded border border-gray-300 bg-gray-100 px-1.5 py-0.5 font-mono dark:border-gray-600 dark:bg-gray-700">
                    ↓
                  </kbd>
                  <span className="ml-1">Navigate</span>
                </div>
                <div className="flex items-center gap-1">
                  <kbd className="rounded border border-gray-300 bg-gray-100 px-1.5 py-0.5 font-mono dark:border-gray-600 dark:bg-gray-700">
                    ↵
                  </kbd>
                  <span className="ml-1">Select</span>
                </div>
                <div className="flex items-center gap-1">
                  <kbd className="rounded border border-gray-300 bg-gray-100 px-1.5 py-0.5 font-mono dark:border-gray-600 dark:bg-gray-700">
                    Esc
                  </kbd>
                  <span className="ml-1">Close</span>
                </div>
              </div>
            </div>
          </ModalBody>
        </div>
      </Modal>
    </>
  );
}
