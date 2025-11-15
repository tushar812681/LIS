'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import {
  Shield,
  Eye,
  Lock,
  FileText,
  UserCheck,
  AlertCircle,
  Mail,
  Clock,
  Globe,
} from "lucide-react";

export default function PrivacyPage() {
  const quickLinks = [
    { title: "Information We Collect", href: "#information-collected" },
    { title: "How We Use Data", href: "#data-usage" },
    { title: "Your Rights", href: "#your-rights" },
    { title: "Data Security", href: "#data-security" },
    { title: "Contact Us", href: "#contact" },
  ];

  const userRights = [
    {
      icon: Eye,
      title: "Right to Access",
      description: "View and obtain a copy of your personal data we hold",
    },
    {
      icon: UserCheck,
      title: "Right to Correction",
      description: "Request correction of inaccurate or incomplete data",
    },
    {
      icon: AlertCircle,
      title: "Right to Erasure",
      description: "Request deletion of your personal data (Right to be Forgotten)",
    },
    {
      icon: Lock,
      title: "Right to Data Portability",
      description: "Receive your data in a structured, machine-readable format",
    },
    {
      icon: Shield,
      title: "Right to Withdraw Consent",
      description: "Withdraw consent for data processing at any time",
    },
    {
      icon: FileText,
      title: "Right to Nominate",
      description: "Nominate another person to exercise rights on your behalf",
    },
  ];

  return (
    <div className="min-h-screen bg-white">
      <Navbar />

      {/* Hero Section */}
      <section className="relative overflow-hidden">
        <div className="absolute inset-0 gradient-mesh opacity-40" />
        <div className="absolute inset-0 grid-pattern" />

        <div className="container-wide section-padding relative">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center max-w-4xl mx-auto space-y-6"
          >
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
              <Shield className="w-4 h-4" />
              DPDP 2023 Compliant
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Privacy <span className="gradient-text">Policy</span>
            </h1>

            <p className="text-xl text-slate-600">
              Last updated: January 7, 2025
            </p>

            <p className="text-lg text-slate-600 leading-relaxed">
              At Floe-LIS, we are committed to protecting your privacy and ensuring the security
              of your personal and health data in compliance with the Digital Personal Data Protection
              Act, 2023 (DPDP Act).
            </p>
          </motion.div>
        </div>
      </section>

      {/* Quick Navigation */}
      <section className="py-8 bg-slate-50 border-y">
        <div className="container-wide">
          <div className="flex flex-wrap justify-center gap-4">
            {quickLinks.map((link, i) => (
              <a
                key={i}
                href={link.href}
                className="px-4 py-2 rounded-full bg-white text-slate-700 text-sm font-medium hover:bg-primary hover:text-white transition-colors"
              >
                {link.title}
              </a>
            ))}
          </div>
        </div>
      </section>

      {/* Main Content */}
      <section className="section-padding">
        <div className="container-wide max-w-4xl">
          <div className="space-y-12">
            {/* Introduction */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">Introduction</h2>
                <div className="prose prose-slate max-w-none">
                  <p className="text-slate-600 leading-relaxed mb-4">
                    Floe-LIS Technologies Private Limited ("we", "us", "our") respects your privacy
                    and is committed to protecting your personal data. This Privacy Policy explains
                    how we collect, use, share, and protect your information when you use our Laboratory
                    Information System.
                  </p>
                  <p className="text-slate-600 leading-relaxed">
                    This policy applies to all users of Floe-LIS, including laboratory administrators,
                    healthcare professionals, patients, and visitors to our website.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Information We Collect */}
            <motion.div
              id="information-collected"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-primary/10">
                    <FileText className="w-6 h-6 text-primary" />
                  </div>
                  <h2 className="text-3xl font-bold">Information We Collect</h2>
                </div>
                <div className="space-y-6">
                  <div>
                    <h3 className="text-xl font-semibold mb-3">Personal Information</h3>
                    <ul className="space-y-2 text-slate-600">
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Name, email address, phone number, and contact details</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Professional information (for healthcare providers)</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Laboratory registration and accreditation details</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Billing and payment information</span>
                      </li>
                    </ul>
                  </div>

                  <div>
                    <h3 className="text-xl font-semibold mb-3">Health Information</h3>
                    <ul className="space-y-2 text-slate-600">
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Patient demographics and medical history</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Laboratory test orders, results, and reports</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Sample collection and tracking information</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>ABDM Health ID and related health data</span>
                      </li>
                    </ul>
                  </div>

                  <div>
                    <h3 className="text-xl font-semibold mb-3">Technical Information</h3>
                    <ul className="space-y-2 text-slate-600">
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>IP address, browser type, and device information</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Usage data, access logs, and system interactions</span>
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="text-primary mt-1">•</span>
                        <span>Cookies and similar tracking technologies</span>
                      </li>
                    </ul>
                  </div>
                </div>
              </GlassCard>
            </motion.div>

            {/* How We Use Data */}
            <motion.div
              id="data-usage"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-purple-500/10">
                    <Lock className="w-6 h-6 text-purple-600" />
                  </div>
                  <h2 className="text-3xl font-bold">How We Use Your Data</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p>We use your personal data only for legitimate purposes with your consent:</p>
                  <ul className="space-y-3">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Service Delivery:</strong> To provide laboratory information management services</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Healthcare Operations:</strong> To facilitate test ordering, processing, and reporting</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Communication:</strong> To send notifications, reports, and service updates</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Compliance:</strong> To meet legal, regulatory, and accreditation requirements</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Analytics:</strong> To improve our services and system performance (anonymized data)</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Security:</strong> To detect, prevent, and address fraud and security issues</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Data Sharing */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">Data Sharing and Disclosure</h2>
                <div className="space-y-4 text-slate-600">
                  <p>We do not sell your personal data. We may share your data only in these circumstances:</p>
                  <ul className="space-y-3">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>With Your Consent:</strong> When you explicitly authorize data sharing</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Healthcare Providers:</strong> With authorized doctors and hospitals for treatment</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>ABDM Network:</strong> Through secure health information exchanges (with consent)</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Service Providers:</strong> With trusted third parties who assist our operations (under strict NDAs)</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Legal Requirements:</strong> When required by law or to protect rights and safety</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Your Rights */}
            <motion.div
              id="your-rights"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <div className="mb-8">
                <h2 className="text-3xl font-bold mb-4 text-center">
                  Your Rights Under <span className="gradient-text">DPDP Act 2023</span>
                </h2>
                <p className="text-slate-600 text-center max-w-2xl mx-auto">
                  You have the following rights regarding your personal data
                </p>
              </div>
              <div className="grid md:grid-cols-2 gap-6">
                {userRights.map((right, i) => (
                  <GlassCard key={i} className="p-6">
                    <div className="flex items-start gap-4">
                      <div className="p-3 rounded-xl bg-primary/10 flex-shrink-0">
                        <right.icon className="w-6 h-6 text-primary" />
                      </div>
                      <div>
                        <h3 className="text-lg font-bold mb-2">{right.title}</h3>
                        <p className="text-slate-600 text-sm">{right.description}</p>
                      </div>
                    </div>
                  </GlassCard>
                ))}
              </div>
              <div className="mt-6">
                <GlassCard variant="strong" className="p-6">
                  <p className="text-slate-700 text-center">
                    To exercise any of these rights, please contact our Data Protection Officer at{" "}
                    <a href="mailto:privacy@floe-lis.com" className="text-primary hover:underline font-medium">
                      privacy@floe-lis.com
                    </a>
                  </p>
                </GlassCard>
              </div>
            </motion.div>

            {/* Data Security */}
            <motion.div
              id="data-security"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-green-500/10">
                    <Shield className="w-6 h-6 text-green-600" />
                  </div>
                  <h2 className="text-3xl font-bold">Data Security</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p>We implement industry-leading security measures to protect your data:</p>
                  <ul className="space-y-3">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>AES-256 encryption for data at rest</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>TLS 1.3 encryption for data in transit</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Role-based access controls and multi-factor authentication</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Regular security audits and penetration testing</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>24/7 security monitoring and incident response</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>ISO 27001 certified information security management</span>
                    </li>
                  </ul>
                  <p className="pt-4">
                    For more details, visit our <Link href="/security" className="text-primary hover:underline font-medium">Security & Compliance</Link> page.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Data Retention */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-orange-500/10">
                    <Clock className="w-6 h-6 text-orange-600" />
                  </div>
                  <h2 className="text-3xl font-bold">Data Retention</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p>We retain your data only as long as necessary:</p>
                  <ul className="space-y-3">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Medical Records:</strong> As per NABL and regulatory requirements (typically 3-5 years)</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Account Data:</strong> Until you request deletion or account closure</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Audit Logs:</strong> As required by compliance standards</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span><strong>Analytics Data:</strong> Anonymized data may be retained for service improvement</span>
                    </li>
                  </ul>
                  <p className="pt-4">
                    After the retention period, data is securely deleted or anonymized.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* International Transfers */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-blue-500/10">
                    <Globe className="w-6 h-6 text-blue-600" />
                  </div>
                  <h2 className="text-3xl font-bold">International Data Transfers</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p>
                    Your data is primarily stored in India. If we need to transfer data internationally
                    (e.g., for backup or service providers), we ensure:
                  </p>
                  <ul className="space-y-3">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Compliance with DPDP Act requirements</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Adequate data protection safeguards</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Your explicit consent when required</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Cookies */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">Cookies and Tracking</h2>
                <div className="space-y-4 text-slate-600">
                  <p>We use cookies and similar technologies to:</p>
                  <ul className="space-y-3">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Maintain your login session</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Remember your preferences</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Analyze site usage and performance</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Provide security features</span>
                    </li>
                  </ul>
                  <p className="pt-4">
                    You can control cookies through your browser settings. However, disabling cookies
                    may affect functionality.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Children's Privacy */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">Children's Privacy</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    We do not knowingly collect personal data from children under 18 without verifiable
                    parental consent. For pediatric patients, we require consent from parents or legal
                    guardians as per DPDP Act requirements.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Changes to Policy */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">Changes to This Policy</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    We may update this Privacy Policy from time to time. We will notify you of
                    significant changes via:
                  </p>
                  <ul className="space-y-2">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Email notification to registered users</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Prominent notice on our platform</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Updated "Last updated" date at the top of this page</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Contact */}
            <motion.div
              id="contact"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard variant="strong" className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-primary/10">
                    <Mail className="w-6 h-6 text-primary" />
                  </div>
                  <h2 className="text-3xl font-bold">Contact Us</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p>For any questions about this Privacy Policy or to exercise your rights:</p>
                  <div className="grid md:grid-cols-2 gap-6 mt-6">
                    <div>
                      <h3 className="font-semibold text-slate-900 mb-2">Data Protection Officer</h3>
                      <p className="text-sm">
                        Email: <a href="mailto:privacy@floe-lis.com" className="text-primary hover:underline">privacy@floe-lis.com</a><br />
                        Phone: +91 80 1234 5678
                      </p>
                    </div>
                    <div>
                      <h3 className="font-semibold text-slate-900 mb-2">Registered Office</h3>
                      <p className="text-sm">
                        Floe-LIS Technologies Private Limited<br />
                        Innovation Hub, MG Road<br />
                        Bangalore, Karnataka 560001<br />
                        India
                      </p>
                    </div>
                  </div>
                </div>
              </GlassCard>
            </motion.div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <GlassCard variant="strong" className="p-12 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-6 max-w-2xl mx-auto"
            >
              <h2 className="text-3xl font-bold">
                Questions About Your Privacy?
              </h2>
              <p className="text-lg text-slate-600">
                Our team is here to help. Contact us for any privacy-related inquiries.
              </p>
              <Link href="/contact">
                <Button size="lg" className="hover-glow">
                  Contact Privacy Team
                </Button>
              </Link>
            </motion.div>
          </GlassCard>
        </div>
      </section>

      <Footer />
    </div>
  );
}
