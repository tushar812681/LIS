'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import {
  FileText,
  Scale,
  UserCheck,
  AlertCircle,
  Shield,
  CreditCard,
  Gavel,
  Mail,
} from "lucide-react";

export default function TermsPage() {
  const quickLinks = [
    { title: "Acceptance of Terms", href: "#acceptance" },
    { title: "User Responsibilities", href: "#responsibilities" },
    { title: "Service Terms", href: "#service-terms" },
    { title: "Payment & Fees", href: "#payment" },
    { title: "Termination", href: "#termination" },
    { title: "Contact", href: "#contact" },
  ];

  const keyTerms = [
    {
      icon: UserCheck,
      title: "Account Registration",
      description: "Valid credentials and accurate information required",
    },
    {
      icon: Shield,
      title: "Data Protection",
      description: "Compliance with healthcare data security standards",
    },
    {
      icon: CreditCard,
      title: "Payment Terms",
      description: "Clear billing cycles and refund policies",
    },
    {
      icon: Scale,
      title: "Legal Compliance",
      description: "Adherence to Indian healthcare regulations",
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
              <FileText className="w-4 h-4" />
              Legal Agreement
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Terms of <span className="gradient-text">Service</span>
            </h1>

            <p className="text-xl text-slate-600">
              Last updated: January 7, 2025
            </p>

            <p className="text-lg text-slate-600 leading-relaxed">
              Please read these Terms of Service carefully before using Floe-LIS.
              By accessing or using our services, you agree to be bound by these terms.
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

      {/* Key Terms Overview */}
      <section className="py-12">
        <div className="container-wide">
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {keyTerms.map((term, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 h-full text-center">
                  <div className="p-3 rounded-xl bg-primary/10 w-fit mx-auto mb-4">
                    <term.icon className="w-6 h-6 text-primary" />
                  </div>
                  <h3 className="font-bold mb-2">{term.title}</h3>
                  <p className="text-slate-600 text-sm">{term.description}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Main Content */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide max-w-4xl">
          <div className="space-y-8">
            {/* Acceptance of Terms */}
            <motion.div
              id="acceptance"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">1. Acceptance of Terms</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    By creating an account, accessing, or using Floe-LIS ("Service"), you agree to be
                    bound by these Terms of Service ("Terms"). If you do not agree to these Terms,
                    please do not use our Service.
                  </p>
                  <p>
                    These Terms constitute a legally binding agreement between you and Floe-LIS
                    Technologies Private Limited ("Floe-LIS", "we", "us", "our").
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Service Description */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">2. Service Description</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    Floe-LIS provides a cloud-based Laboratory Information System designed to manage
                    laboratory operations, including:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Patient registration and management</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Sample tracking and processing</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Test result management and reporting</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Integration with ABDM and other healthcare systems</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Analytics and reporting tools</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* User Accounts */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">3. User Accounts and Registration</h2>
                <div className="space-y-4 text-slate-600">
                  <p>To use our Service, you must:</p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Be at least 18 years old or have parental/guardian consent</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Provide accurate, current, and complete registration information</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Maintain and update your information to keep it accurate</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Maintain the security of your account credentials</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Accept responsibility for all activities under your account</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Notify us immediately of any unauthorized access</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* User Responsibilities */}
            <motion.div
              id="responsibilities"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">4. User Responsibilities</h2>
                <div className="space-y-4 text-slate-600">
                  <p><strong>You agree to:</strong></p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Use the Service only for lawful purposes</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Comply with all applicable healthcare regulations and laws</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Maintain proper licensing and accreditation for your laboratory</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Ensure accuracy of data entered into the system</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Obtain necessary patient consents for data processing</span>
                    </li>
                  </ul>
                  <p className="pt-4"><strong>You agree NOT to:</strong></p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Violate any laws, regulations, or third-party rights</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Attempt to gain unauthorized access to the Service</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Interfere with or disrupt the Service or servers</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Use the Service to transmit malware or harmful code</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Reverse engineer or attempt to extract source code</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Share your account credentials with unauthorized parties</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Intellectual Property */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">5. Intellectual Property</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    The Service, including its original content, features, and functionality, is owned
                    by Floe-LIS and is protected by international copyright, trademark, patent, and
                    other intellectual property laws.
                  </p>
                  <p>
                    You retain all rights to the data you input into the Service. By using the Service,
                    you grant us a limited license to use, store, and process your data solely to
                    provide the Service to you.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Service Terms */}
            <motion.div
              id="service-terms"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">6. Service Availability and Modifications</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    We strive to maintain 99.99% uptime but do not guarantee uninterrupted service.
                    We may:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Perform scheduled maintenance with advance notice</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Modify, suspend, or discontinue features with reasonable notice</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Update the Service to improve functionality or security</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Temporarily suspend access for security or technical reasons</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Payment and Fees */}
            <motion.div
              id="payment"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">7. Payment and Fees</h2>
                <div className="space-y-4 text-slate-600">
                  <p><strong>Subscription Plans:</strong></p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Fees are based on your selected subscription plan</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>All prices are in Indian Rupees (INR) and exclusive of applicable taxes</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Payments are billed in advance on a monthly or annual basis</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Free trial period does not require payment information</span>
                    </li>
                  </ul>
                  <p className="pt-4"><strong>Refund Policy:</strong></p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Annual subscriptions: Pro-rated refund within 30 days</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Monthly subscriptions: No refunds after billing cycle starts</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Enterprise plans: As per custom agreement</span>
                    </li>
                  </ul>
                  <p className="pt-4"><strong>Payment Terms:</strong></p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>You authorize us to charge your payment method automatically</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Failed payments may result in service suspension</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Price changes will be notified 30 days in advance</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Termination */}
            <motion.div
              id="termination"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">8. Termination</h2>
                <div className="space-y-4 text-slate-600">
                  <p><strong>By You:</strong></p>
                  <p>
                    You may cancel your subscription at any time through your account settings. Upon
                    cancellation, you will have access until the end of your current billing period.
                  </p>
                  <p className="pt-4"><strong>By Us:</strong></p>
                  <p>
                    We may suspend or terminate your access immediately if you:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Violate these Terms or applicable laws</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Fail to pay fees when due</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Engage in fraudulent or illegal activities</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Pose a security risk to the Service</span>
                    </li>
                  </ul>
                  <p className="pt-4"><strong>Effect of Termination:</strong></p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Your right to use the Service ceases immediately</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Data export available for 30 days post-termination</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>After 90 days, data may be permanently deleted</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Disclaimers */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">9. Disclaimers and Warranties</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    THE SERVICE IS PROVIDED "AS IS" AND "AS AVAILABLE" WITHOUT WARRANTIES OF ANY KIND,
                    EITHER EXPRESS OR IMPLIED.
                  </p>
                  <p>
                    We do not warrant that:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>The Service will be error-free or uninterrupted</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Defects will be corrected immediately</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>The Service will meet your specific requirements</span>
                    </li>
                  </ul>
                  <p className="pt-4">
                    <strong>Medical Disclaimer:</strong> Floe-LIS is a management tool only. We do not
                    provide medical advice, diagnosis, or treatment. All medical decisions remain the
                    responsibility of qualified healthcare professionals.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Limitation of Liability */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">10. Limitation of Liability</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    TO THE MAXIMUM EXTENT PERMITTED BY LAW, FLOE-LIS SHALL NOT BE LIABLE FOR:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Indirect, incidental, special, or consequential damages</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Loss of profits, data, or business opportunities</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Service interruptions or technical failures</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Unauthorized access or data breaches beyond our control</span>
                    </li>
                  </ul>
                  <p className="pt-4">
                    Our total liability shall not exceed the amount you paid us in the 12 months
                    preceding the claim.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Indemnification */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">11. Indemnification</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    You agree to indemnify and hold harmless Floe-LIS and its officers, directors,
                    employees, and agents from any claims, damages, losses, liabilities, and expenses
                    arising from:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Your use of the Service</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Your violation of these Terms</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Your violation of any rights of another party</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Data you input or actions you take using the Service</span>
                    </li>
                  </ul>
                </div>
              </GlassCard>
            </motion.div>

            {/* Dispute Resolution */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <div className="flex items-center gap-3 mb-6">
                  <div className="p-3 rounded-xl bg-primary/10">
                    <Gavel className="w-6 h-6 text-primary" />
                  </div>
                  <h2 className="text-3xl font-bold">12. Dispute Resolution and Governing Law</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p><strong>Governing Law:</strong></p>
                  <p>
                    These Terms shall be governed by and construed in accordance with the laws of
                    India, without regard to conflict of law principles.
                  </p>
                  <p className="pt-4"><strong>Jurisdiction:</strong></p>
                  <p>
                    Any disputes arising from these Terms or the Service shall be subject to the
                    exclusive jurisdiction of the courts in Bangalore, Karnataka, India.
                  </p>
                  <p className="pt-4"><strong>Dispute Resolution:</strong></p>
                  <p>
                    Before filing any legal claim, parties agree to attempt resolution through good
                    faith negotiation for at least 30 days.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Changes to Terms */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">13. Changes to Terms</h2>
                <div className="space-y-4 text-slate-600">
                  <p>
                    We may revise these Terms at any time. Material changes will be notified:
                  </p>
                  <ul className="space-y-2 ml-6">
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Via email to registered users</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>Through prominent notice in the Service</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-primary mt-1">•</span>
                      <span>At least 30 days before changes take effect</span>
                    </li>
                  </ul>
                  <p className="pt-4">
                    Continued use of the Service after changes constitutes acceptance of the revised Terms.
                  </p>
                </div>
              </GlassCard>
            </motion.div>

            {/* Miscellaneous */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-4">14. Miscellaneous</h2>
                <div className="space-y-4 text-slate-600">
                  <p><strong>Entire Agreement:</strong></p>
                  <p>
                    These Terms, together with our Privacy Policy, constitute the entire agreement
                    between you and Floe-LIS.
                  </p>
                  <p className="pt-4"><strong>Severability:</strong></p>
                  <p>
                    If any provision is found unenforceable, the remaining provisions continue in full
                    force and effect.
                  </p>
                  <p className="pt-4"><strong>Waiver:</strong></p>
                  <p>
                    Failure to enforce any right or provision does not constitute a waiver of such
                    right or provision.
                  </p>
                  <p className="pt-4"><strong>Assignment:</strong></p>
                  <p>
                    You may not assign these Terms without our written consent. We may assign our
                    rights and obligations without restriction.
                  </p>
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
                  <h2 className="text-3xl font-bold">Contact Information</h2>
                </div>
                <div className="space-y-4 text-slate-600">
                  <p>For questions about these Terms, please contact:</p>
                  <div className="grid md:grid-cols-2 gap-6 mt-6">
                    <div>
                      <h3 className="font-semibold text-slate-900 mb-2">Legal Department</h3>
                      <p className="text-sm">
                        Email: <a href="mailto:legal@floe-lis.com" className="text-primary hover:underline">legal@floe-lis.com</a><br />
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
      <section className="section-padding">
        <div className="container-wide">
          <GlassCard variant="strong" className="p-12 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-6 max-w-2xl mx-auto"
            >
              <AlertCircle className="w-12 h-12 text-primary mx-auto" />
              <h2 className="text-3xl font-bold">
                Questions About These Terms?
              </h2>
              <p className="text-lg text-slate-600">
                Our legal team is available to answer your questions.
              </p>
              <Link href="/contact">
                <Button size="lg" className="hover-glow">
                  Contact Legal Team
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
