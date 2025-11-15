'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import {
  Shield,
  Lock,
  Server,
  Eye,
  FileCheck,
  AlertTriangle,
  Clock,
  CheckCircle,
  Database,
  Key,
  Activity,
  FileText,
  Users,
  Fingerprint,
} from "lucide-react";

export default function SecurityPage() {
  const certifications = [
    {
      icon: Shield,
      title: "NABL Accredited",
      description: "National Accreditation Board for Testing and Calibration Laboratories compliant",
      color: "blue",
    },
    {
      icon: FileCheck,
      title: "ISO 15189:2022",
      description: "Medical laboratories - Requirements for quality and competence",
      color: "green",
    },
    {
      icon: Lock,
      title: "ISO 27001",
      description: "Information security management systems certified",
      color: "purple",
    },
    {
      icon: Shield,
      title: "HIPAA Compliant",
      description: "Health Insurance Portability and Accountability Act compliant",
      color: "red",
    },
    {
      icon: Database,
      title: "ABDM Integrated",
      description: "Ayushman Bharat Digital Mission integrated and certified",
      color: "orange",
    },
    {
      icon: FileText,
      title: "DPDP 2023",
      description: "Digital Personal Data Protection Act 2023 compliant",
      color: "teal",
    },
  ];

  const securityFeatures = [
    {
      icon: Lock,
      title: "End-to-End Encryption",
      description: "AES-256 encryption for data at rest and TLS 1.3 for data in transit",
      details: [
        "Military-grade AES-256 bit encryption",
        "TLS 1.3 for all data transmission",
        "Encrypted database backups",
        "Secure key management system",
      ],
    },
    {
      icon: Key,
      title: "Access Control",
      description: "Role-based access control with multi-factor authentication",
      details: [
        "Role-based permissions (RBAC)",
        "Multi-factor authentication (MFA)",
        "Single sign-on (SSO) support",
        "IP whitelisting and geofencing",
      ],
    },
    {
      icon: Activity,
      title: "Audit Trails",
      description: "Comprehensive logging of all system activities and data changes",
      details: [
        "Complete audit logs for all actions",
        "Tamper-proof log storage",
        "Real-time monitoring and alerts",
        "Compliance reporting tools",
      ],
    },
    {
      icon: Database,
      title: "Data Backup",
      description: "Automated daily backups with 99.99% uptime guarantee",
      details: [
        "Automated hourly incremental backups",
        "Daily full system backups",
        "Geo-redundant storage across 3 regions",
        "Point-in-time recovery capability",
      ],
    },
    {
      icon: Server,
      title: "Infrastructure Security",
      description: "Enterprise-grade cloud infrastructure with DDoS protection",
      details: [
        "AWS/Azure certified data centers",
        "DDoS protection and WAF",
        "Network segmentation and firewalls",
        "Regular penetration testing",
      ],
    },
    {
      icon: Fingerprint,
      title: "Privacy by Design",
      description: "Built-in privacy controls and data minimization principles",
      details: [
        "Data anonymization capabilities",
        "Right to erasure (RTBF)",
        "Consent management system",
        "Privacy impact assessments",
      ],
    },
  ];

  const complianceProcess = [
    {
      step: "01",
      title: "Data Collection",
      description: "Only collect necessary data with explicit user consent",
      icon: Users,
    },
    {
      step: "02",
      title: "Secure Storage",
      description: "Encrypted storage in ISO-certified data centers",
      icon: Database,
    },
    {
      step: "03",
      title: "Access Control",
      description: "Strict role-based access with MFA and audit trails",
      icon: Key,
    },
    {
      step: "04",
      title: "Regular Audits",
      description: "Quarterly security audits and compliance reviews",
      icon: FileCheck,
    },
  ];

  const stats = [
    { value: "99.99%", label: "Uptime SLA" },
    { value: "< 2 hours", label: "Incident Response" },
    { value: "24/7", label: "Security Monitoring" },
    { value: "Zero", label: "Data Breaches" },
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
              Enterprise-Grade Security
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Security & <span className="gradient-text">Compliance</span>
            </h1>

            <p className="text-xl text-slate-600 leading-relaxed">
              Your data security and patient privacy are our top priorities. We maintain the highest
              standards of security, compliance, and data protection in the healthcare industry.
            </p>

            <div className="flex flex-col sm:flex-row gap-4 justify-center pt-4">
              <Link href="/contact">
                <Button size="lg" className="hover-glow">
                  <FileText className="w-5 h-5 mr-2" />
                  Security Documentation
                </Button>
              </Link>
              <Link href="/demo">
                <Button size="lg" variant="outline" className="hover-lift">
                  Schedule Security Review
                </Button>
              </Link>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-12 bg-slate-50">
        <div className="container-wide">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
            {stats.map((stat, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 text-center">
                  <div className="text-3xl md:text-4xl font-bold text-primary mb-2">
                    {stat.value}
                  </div>
                  <p className="text-sm text-slate-600">{stat.label}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Certifications */}
      <section className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Industry <span className="gradient-text">Certifications</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              Certified and compliant with international healthcare and security standards
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {certifications.map((cert, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 h-full">
                  <div className={`p-3 rounded-xl bg-${cert.color}-500/10 w-fit mb-4`}>
                    <cert.icon className={`w-8 h-8 text-${cert.color}-600`} />
                  </div>
                  <h3 className="text-xl font-bold mb-2">{cert.title}</h3>
                  <p className="text-slate-600 text-sm">{cert.description}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Security Features */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Multi-Layer <span className="gradient-text">Security</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              Comprehensive security measures protecting your data at every level
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 gap-6">
            {securityFeatures.map((feature, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-8 h-full">
                  <div className="flex items-start gap-4 mb-4">
                    <div className="p-3 rounded-xl bg-primary/10 flex-shrink-0">
                      <feature.icon className="w-6 h-6 text-primary" />
                    </div>
                    <div>
                      <h3 className="text-xl font-bold mb-2">{feature.title}</h3>
                      <p className="text-slate-600 text-sm">{feature.description}</p>
                    </div>
                  </div>
                  <ul className="space-y-2 ml-14">
                    {feature.details.map((detail, j) => (
                      <li key={j} className="flex items-center gap-2 text-sm text-slate-700">
                        <CheckCircle className="w-4 h-4 text-primary flex-shrink-0" />
                        {detail}
                      </li>
                    ))}
                  </ul>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Compliance Process */}
      <section className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Compliance <span className="gradient-text">Process</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              How we ensure data protection and regulatory compliance
            </p>
          </motion.div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {complianceProcess.map((process, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 h-full text-center">
                  <div className="p-3 rounded-xl bg-primary/10 w-fit mx-auto mb-4">
                    <process.icon className="w-8 h-8 text-primary" />
                  </div>
                  <div className="text-3xl font-bold text-primary mb-3">{process.step}</div>
                  <h3 className="text-lg font-bold mb-2">{process.title}</h3>
                  <p className="text-slate-600 text-sm">{process.description}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Incident Response */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <div className="grid lg:grid-cols-2 gap-12 items-center">
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
            >
              <div className="space-y-6">
                <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-red-50 text-red-600 text-sm font-medium">
                  <AlertTriangle className="w-4 h-4" />
                  24/7 Security Operations
                </div>
                <h2 className="text-4xl font-bold">
                  Rapid <span className="gradient-text">Incident Response</span>
                </h2>
                <p className="text-lg text-slate-600">
                  Our dedicated security operations center monitors systems 24/7 with automated
                  threat detection and immediate response protocols.
                </p>
                <div className="space-y-4">
                  {[
                    "Real-time threat detection and monitoring",
                    "Automated incident response workflows",
                    "< 2 hour response time guarantee",
                    "Post-incident analysis and reporting",
                    "Regular security training for all staff",
                  ].map((item, i) => (
                    <div key={i} className="flex items-start gap-3">
                      <CheckCircle className="w-5 h-5 text-primary flex-shrink-0 mt-0.5" />
                      <span className="text-slate-700">{item}</span>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard variant="strong" className="p-8">
                <div className="space-y-6">
                  <div className="text-center">
                    <Clock className="w-16 h-16 text-primary mx-auto mb-4" />
                    <h3 className="text-2xl font-bold mb-2">Security Operations Center</h3>
                    <p className="text-slate-600">
                      Round-the-clock monitoring and instant response
                    </p>
                  </div>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="text-center p-4 rounded-xl bg-white/50">
                      <div className="text-2xl font-bold text-primary mb-1">24/7</div>
                      <div className="text-sm text-slate-600">Monitoring</div>
                    </div>
                    <div className="text-center p-4 rounded-xl bg-white/50">
                      <div className="text-2xl font-bold text-primary mb-1">< 2h</div>
                      <div className="text-sm text-slate-600">Response Time</div>
                    </div>
                    <div className="text-center p-4 rounded-xl bg-white/50">
                      <div className="text-2xl font-bold text-primary mb-1">100%</div>
                      <div className="text-sm text-slate-600">Threat Coverage</div>
                    </div>
                    <div className="text-center p-4 rounded-xl bg-white/50">
                      <div className="text-2xl font-bold text-primary mb-1">15min</div>
                      <div className="text-sm text-slate-600">Alert Time</div>
                    </div>
                  </div>
                </div>
              </GlassCard>
            </motion.div>
          </div>
        </div>
      </section>

      {/* Trust & Transparency */}
      <section className="section-padding">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-12"
          >
            <h2 className="text-4xl font-bold mb-4">
              Trust & <span className="gradient-text">Transparency</span>
            </h2>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              We believe in complete transparency about our security practices
            </p>
          </motion.div>

          <div className="grid md:grid-cols-3 gap-6">
            {[
              {
                icon: Eye,
                title: "Transparent Operations",
                description: "Regular security updates and transparent incident reporting",
              },
              {
                icon: FileCheck,
                title: "Third-Party Audits",
                description: "Annual independent security audits by certified firms",
              },
              {
                icon: Users,
                title: "Customer Control",
                description: "Full data portability and deletion controls for customers",
              },
            ].map((item, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 h-full text-center">
                  <div className="p-3 rounded-xl bg-primary/10 w-fit mx-auto mb-4">
                    <item.icon className="w-8 h-8 text-primary" />
                  </div>
                  <h3 className="text-xl font-bold mb-2">{item.title}</h3>
                  <p className="text-slate-600 text-sm">{item.description}</p>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <GlassCard variant="strong" className="p-12 md:p-16 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-6 max-w-2xl mx-auto"
            >
              <Shield className="w-12 h-12 text-primary mx-auto" />

              <h2 className="text-4xl font-bold">
                Questions About Our Security?
              </h2>

              <p className="text-xl text-slate-600">
                Our security team is happy to answer any questions and provide detailed documentation.
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
                <Link href="/contact">
                  <Button size="lg" className="hover-glow">
                    Contact Security Team
                  </Button>
                </Link>
                <Link href="/demo">
                  <Button size="lg" variant="outline" className="hover-lift">
                    Schedule Security Review
                  </Button>
                </Link>
              </div>
            </motion.div>
          </GlassCard>
        </div>
      </section>

      <Footer />
    </div>
  );
}
