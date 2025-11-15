'use client';

import * as React from 'react';
import { Check, ChevronsUpDown } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';

const countries = [
  { code: '+91', country: 'IN', name: 'India', flag: '🇮🇳', placeholder: '9812345678', format: 'XXXXXXXXXX' },
  { code: '+1', country: 'US', name: 'United States', flag: '🇺🇸', placeholder: '(555) 123-4567', format: '(XXX) XXX-XXXX' },
  { code: '+44', country: 'GB', name: 'United Kingdom', flag: '🇬🇧', placeholder: '07700 900000', format: 'XXXXX XXXXXX' },
  { code: '+61', country: 'AU', name: 'Australia', flag: '🇦🇺', placeholder: '0412 345 678', format: 'XXXX XXX XXX' },
  { code: '+81', country: 'JP', name: 'Japan', flag: '🇯🇵', placeholder: '090-1234-5678', format: 'XXX-XXXX-XXXX' },
  { code: '+86', country: 'CN', name: 'China', flag: '🇨🇳', placeholder: '138 0013 8000', format: 'XXX XXXX XXXX' },
  { code: '+49', country: 'DE', name: 'Germany', flag: '🇩🇪', placeholder: '151 23456789', format: 'XXX XXXXXXXX' },
  { code: '+33', country: 'FR', name: 'France', flag: '🇫🇷', placeholder: '06 12 34 56 78', format: 'XX XX XX XX XX' },
  { code: '+39', country: 'IT', name: 'Italy', flag: '🇮🇹', placeholder: '312 345 6789', format: 'XXX XXX XXXX' },
  { code: '+34', country: 'ES', name: 'Spain', flag: '🇪🇸', placeholder: '612 34 56 78', format: 'XXX XX XX XX' },
  { code: '+7', country: 'RU', name: 'Russia', flag: '🇷🇺', placeholder: '912 345-67-89', format: 'XXX XXX-XX-XX' },
  { code: '+55', country: 'BR', name: 'Brazil', flag: '🇧🇷', placeholder: '(11) 91234-5678', format: '(XX) XXXXX-XXXX' },
  { code: '+52', country: 'MX', name: 'Mexico', flag: '🇲🇽', placeholder: '55 1234 5678', format: 'XX XXXX XXXX' },
  { code: '+82', country: 'KR', name: 'South Korea', flag: '🇰🇷', placeholder: '010-1234-5678', format: 'XXX-XXXX-XXXX' },
  { code: '+27', country: 'ZA', name: 'South Africa', flag: '🇿🇦', placeholder: '071 123 4567', format: 'XXX XXX XXXX' },
  { code: '+971', country: 'AE', name: 'UAE', flag: '🇦🇪', placeholder: '50 123 4567', format: 'XX XXX XXXX' },
  { code: '+65', country: 'SG', name: 'Singapore', flag: '🇸🇬', placeholder: '8123 4567', format: 'XXXX XXXX' },
  { code: '+60', country: 'MY', name: 'Malaysia', flag: '🇲🇾', placeholder: '012-345 6789', format: 'XXX-XXX XXXX' },
  { code: '+66', country: 'TH', name: 'Thailand', flag: '🇹🇭', placeholder: '081 234 5678', format: 'XXX XXX XXXX' },
  { code: '+62', country: 'ID', name: 'Indonesia', flag: '🇮🇩', placeholder: '0812-3456-7890', format: 'XXXX-XXXX-XXXX' },
];

interface PhoneInputProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function PhoneInput({ value, onChange, disabled }: PhoneInputProps) {
  const [open, setOpen] = React.useState(false);
  const [selectedCountry, setSelectedCountry] = React.useState(countries[0]); // Default to India
  const [phoneNumber, setPhoneNumber] = React.useState('');

  // Parse existing value on mount and when value changes
  React.useEffect(() => {
    if (value && value.startsWith('+')) {
      // Find matching country code (sort by code length descending to match longer codes first)
      const sortedCountries = [...countries].sort((a, b) => b.code.length - a.code.length);
      const country = sortedCountries.find(c => value.startsWith(c.code));
      if (country) {
        setSelectedCountry(country);
        setPhoneNumber(value.substring(country.code.length).trim());
      }
    }
  }, [value]);

  const handlePhoneChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newPhone = e.target.value.replace(/[^\d]/g, ''); // Only allow digits
    setPhoneNumber(newPhone);
    // Update full value (no space for E.164 format)
    const fullValue = newPhone ? `${selectedCountry.code}${newPhone}` : selectedCountry.code;
    onChange(fullValue);
  };

  return (
    <div className="flex gap-2">
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            variant="outline"
            role="combobox"
            aria-expanded={open}
            disabled={disabled}
            className="w-[140px] justify-between"
            type="button"
          >
            <span className="flex items-center gap-2">
              <span className="text-lg">{selectedCountry.flag}</span>
              <span>{selectedCountry.code}</span>
            </span>
            <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[280px] p-0" align="start">
          <Command>
            <CommandInput placeholder="Search country..." />
            <CommandEmpty>No country found.</CommandEmpty>
            <CommandGroup className="max-h-[300px] overflow-auto">
              {countries.map((country) => (
                <CommandItem
                  key={country.country}
                  value={`${country.name} ${country.code}`}
                  onSelect={(currentValue) => {
                    const selected = countries.find(c =>
                      currentValue.toLowerCase().includes(c.name.toLowerCase())
                    );
                    if (selected) {
                      setSelectedCountry(selected);
                      setOpen(false);
                      const fullValue = phoneNumber ? `${selected.code}${phoneNumber}` : selected.code;
                      onChange(fullValue);
                    }
                  }}
                >
                  <Check
                    className={cn(
                      'mr-2 h-4 w-4',
                      selectedCountry.country === country.country ? 'opacity-100' : 'opacity-0'
                    )}
                  />
                  <span className="mr-2 text-lg">{country.flag}</span>
                  <span className="flex-1">{country.name}</span>
                  <span className="text-muted-foreground">{country.code}</span>
                </CommandItem>
              ))}
            </CommandGroup>
          </Command>
        </PopoverContent>
      </Popover>
      <Input
        type="tel"
        placeholder={selectedCountry.placeholder}
        value={phoneNumber}
        onChange={handlePhoneChange}
        disabled={disabled}
        className="flex-1"
      />
    </div>
  );
}
