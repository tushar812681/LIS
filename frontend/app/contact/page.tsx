'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { Mail, Phone, MapPin, Clock, Send, MessageSquare, Headphones } from "lucide-react";
import { useState } from "react";

export default function ContactPage() {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    company: '',
    phone: '',
    message: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Handle form submission
    console.log('Form submitted:', formData);
  };

  const contactInfo = [
    {
      icon: Mail,
      title: "Email",
      value: "contact@floe-lis.com",
      link: "mailto:contact@floe-lis.com",
      color: "blue",
    },
    {
      icon: Phone,
      title: "Phone",
      value: "+91 80 1234 5678",
      link: "tel:+918012345678",
      color: "green",
    },
    {
      icon: MapPin,
      title: "Office",
      value: "Bangalore, Karnataka, India",
      link: null,
      color: "purple",
    },
    {
      icon: Clock,
      title: "Business Hours",
      value: "Mon-Fri: 9AM - 6PM IST",
      link: null,
      color: "orange",
    },
  ];

  const supportOptions = [
    {
      icon: Headphones,
      title: "Technical Support",
      description: "Get help with technical issues and troubleshooting",
      action: "Open Support Ticket",
      link: "/support",
    },
    {
      icon: MessageSquare,
      title: "Sales Inquiry",
      description: "Questions about pricing, features, or demos",
      action: "Talk to Sales",
      link: "/contact?type=sales",
    },
    {
      icon: Mail,
      title: "General Questions",
      description: "Any other questions or feedback",
      action: "Send Message",
      link: "#contact-form",
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
            className="text-center max-w-3xl mx-auto space-y-6"
          >
            <div className="inline-block px-4 py-2 rounded-full bg-primary/10 text-primary text-sm font-medium">
              We're here to help
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Get in <span className="gradient-text">Touch</span>
            </h1>

            <p className="text-xl text-slate-600">
              Have questions? We'd love to hear from you. Send us a message and we'll respond as soon as possible.
            </p>
          </motion.div>
        </div>
      </section>

      {/* Contact Options */}
      <section className="py-12 bg-slate-50">
        <div className="container-wide">
          <div className="grid md:grid-cols-3 gap-6">
            {supportOptions.map((option, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.1 }}
              >
                <GlassCard className="p-6 h-full">
                  <div className="space-y-4">
                    <div className="p-3 rounded-xl bg-primary/10 w-fit">
                      <option.icon className="w-6 h-6 text-primary" />
                    </div>
                    <h3 className="text-xl font-bold">{option.title}</h3>
                    <p className="text-slate-600 text-sm">{option.description}</p>
                    <Link href={option.link}>
                      <Button variant="outline" className="w-full hover-lift">
                        {option.action}
                      </Button>
                    </Link>
                  </div>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Main Contact Section */}
      <section id="contact-form" className="section-padding">
        <div className="container-wide">
          <div className="grid lg:grid-cols-2 gap-12">
            {/* Contact Form */}
            <motion.div
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
            >
              <GlassCard className="p-8">
                <h2 className="text-3xl font-bold mb-6">Send us a message</h2>

                <form onSubmit={handleSubmit} className="space-y-6">
                  <div>
                    <label className="block text-sm font-medium mb-2">
                      Full Name *
                    </label>
                    <Input
                      type="text"
                      placeholder="John Doe"
                      value={formData.name}
                      onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                      required
                      className="w-full"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium mb-2">
                      Email Address *
                    </label>
                    <Input
                      type="email"
                      placeholder="john@example.com"
                      value={formData.email}
                      onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                      required
                      className="w-full"
                    />
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div>
                      <label className="block text-sm font-medium mb-2">
                        Company
                      </label>
                      <Input
                        type="text"
                        placeholder="Your Lab Name"
                        value={formData.company}
                        onChange={(e) => setFormData({ ...formData, company: e.target.value })}
                        className="w-full"
                      />
                    </div>

                    <div>
                      <label className="block text-sm font-medium mb-2">
                        Phone Number
                      </label>
                      <Input
                        type="tel"
                        placeholder="+91 98765 43210"
                        value={formData.phone}
                        onChange={(e) => setFormData({ ...formData, phone: e.target.value })}
                        className="w-full"
                      />
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium mb-2">
                      Message *
                    </label>
                    <textarea
                      placeholder="Tell us how we can help..."
                      value={formData.message}
                      onChange={(e) => setFormData({ ...formData, message: e.target.value })}
                      required
                      rows={6}
                      className="w-full px-4 py-3 rounded-lg border border-slate-300 focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                    />
                  </div>

                  <Button type="submit" size="lg" className="w-full hover-glow">
                    Send Message
                    <Send className="w-4 h-4 ml-2" />
                  </Button>
                </form>
              </GlassCard>
            </motion.div>

            {/* Contact Info */}
            <motion.div
              initial={{ opacity: 0, x: 20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              className="space-y-6"
            >
              <div>
                <h2 className="text-3xl font-bold mb-2">Contact Information</h2>
                <p className="text-slate-600">
                  Choose your preferred way to get in touch with us.
                </p>
              </div>

              <div className="space-y-4">
                {contactInfo.map((info, i) => (
                  <GlassCard key={i} className="p-6">
                    <div className="flex items-start gap-4">
                      <div className={`p-3 rounded-xl bg-${info.color}-500/10 flex-shrink-0`}>
                        <info.icon className={`w-6 h-6 text-${info.color}-600`} />
                      </div>
                      <div>
                        <h3 className="font-semibold mb-1">{info.title}</h3>
                        {info.link ? (
                          <a
                            href={info.link}
                            className="text-primary hover:underline"
                          >
                            {info.value}
                          </a>
                        ) : (
                          <p className="text-slate-600">{info.value}</p>
                        )}
                      </div>
                    </div>
                  </GlassCard>
                ))}
              </div>

              {/* Additional Info */}
              <GlassCard variant="strong" className="p-6">
                <h3 className="font-semibold mb-3">Need Immediate Assistance?</h3>
                <p className="text-slate-600 text-sm mb-4">
                  Our support team is available during business hours. For urgent issues, please call us directly.
                </p>
                <div className="flex gap-3">
                  <Button size="sm" variant="outline" className="flex-1">
                    Call Now
                  </Button>
                  <Button size="sm" className="flex-1 hover-glow">
                    Live Chat
                  </Button>
                </div>
              </GlassCard>

              {/* Map or Additional Info */}
              <GlassCard className="p-6">
                <h3 className="font-semibold mb-3">Headquarters</h3>
                <p className="text-slate-600 text-sm mb-3">
                  Floe-LIS Technologies Private Limited<br />
                  Innovation Hub, MG Road<br />
                  Bangalore, Karnataka 560001<br />
                  India
                </p>
                <div className="w-full h-48 bg-slate-100 rounded-lg flex items-center justify-center">
                  <MapPin className="w-8 h-8 text-slate-400" />
                </div>
              </GlassCard>
            </motion.div>
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
              <h2 className="text-4xl font-bold">
                Prefer a Product Demo?
              </h2>

              <p className="text-xl text-slate-600">
                See Floe-LIS in action with a personalized demo from our team.
              </p>

              <Link href="/demo">
                <Button size="lg" className="hover-glow">
                  Schedule a Demo
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
