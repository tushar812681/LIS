'use client';

import Link from "next/link";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import { GlassCard } from "@/components/ui/glass-card";
import { Navbar } from "@/components/layout/Navbar";
import { Footer } from "@/components/layout/Footer";
import { Check, X, ArrowRight, Shield, Zap, Users, Star } from "lucide-react";
import { useState } from "react";

export default function PricingPage() {
  const [billingCycle, setBillingCycle] = useState<'monthly' | 'annual'>('monthly');

  const plans = [
    {
      name: "Starter",
      description: "Perfect for small labs getting started",
      monthlyPrice: 9999,
      annualPrice: 99999,
      features: [
        "Up to 100 patients/month",
        "Basic sample tracking",
        "5 user accounts",
        "Email support",
        "NABL compliance tools",
        "Basic reports",
        "WhatsApp notifications",
        "Mobile app access",
      ],
      notIncluded: [
        "AI auto-verification",
        "ABDM integration",
        "Custom workflows",
        "Priority support",
      ],
      highlight: false,
      icon: Users,
      color: "blue",
    },
    {
      name: "Professional",
      description: "For growing laboratories",
      monthlyPrice: 24999,
      annualPrice: 249999,
      features: [
        "Up to 1000 patients/month",
        "Advanced sample tracking",
        "15 user accounts",
        "Priority support",
        "AI auto-verification (30%)",
        "NABL & ISO compliance",
        "ABDM Health ID integration",
        "Advanced analytics",
        "Custom report builder",
        "API access",
        "WhatsApp Business API",
        "Mobile app access",
      ],
      notIncluded: [
        "Dedicated account manager",
        "Custom integrations",
      ],
      highlight: true,
      icon: Zap,
      color: "primary",
    },
    {
      name: "Enterprise",
      description: "For large labs and chains",
      monthlyPrice: null,
      annualPrice: null,
      features: [
        "Unlimited patients",
        "Unlimited users",
        "AI auto-verification (60%)",
        "Full ABDM integration",
        "Custom workflows",
        "Dedicated account manager",
        "24/7 phone support",
        "Custom integrations",
        "Multi-location support",
        "Advanced security",
        "SLA guarantee",
        "Training & onboarding",
        "White-label options",
        "On-premise deployment",
      ],
      notIncluded: [],
      highlight: false,
      icon: Star,
      color: "purple",
    },
  ];

  const savings = (monthlyPrice: number, annualPrice: number) => {
    const monthlyCost = monthlyPrice * 12;
    const saved = monthlyCost - annualPrice;
    const percentage = Math.round((saved / monthlyCost) * 100);
    return percentage;
  };

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
              Simple, transparent pricing
            </div>

            <h1 className="text-5xl md:text-6xl font-bold">
              Choose the <span className="gradient-text">Perfect Plan</span>
            </h1>

            <p className="text-xl text-slate-600">
              Start free, scale as you grow. All plans include core features with no hidden fees.
            </p>

            {/* Billing Toggle */}
            <div className="flex items-center justify-center gap-4 pt-4">
              <span className={`text-sm font-medium ${billingCycle === 'monthly' ? 'text-primary' : 'text-slate-600'}`}>
                Monthly
              </span>
              <button
                onClick={() => setBillingCycle(billingCycle === 'monthly' ? 'annual' : 'monthly')}
                className="relative w-16 h-8 rounded-full bg-slate-200 transition-colors hover:bg-slate-300"
              >
                <motion.div
                  className="absolute top-1 left-1 w-6 h-6 rounded-full bg-primary shadow-lg"
                  animate={{ x: billingCycle === 'annual' ? 32 : 0 }}
                  transition={{ type: 'spring', stiffness: 500, damping: 30 }}
                />
              </button>
              <span className={`text-sm font-medium ${billingCycle === 'annual' ? 'text-primary' : 'text-slate-600'}`}>
                Annual
                <span className="ml-2 px-2 py-0.5 rounded-full bg-green-100 text-green-700 text-xs">
                  Save 17%
                </span>
              </span>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Pricing Cards */}
      <section className="py-20 relative">
        <div className="container-wide">
          <div className="grid lg:grid-cols-3 gap-8">
            {plans.map((plan, index) => (
              <motion.div
                key={plan.name}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.1 }}
              >
                <GlassCard
                  variant={plan.highlight ? 'strong' : 'default'}
                  className={`p-8 h-full flex flex-col ${
                    plan.highlight ? 'ring-2 ring-primary shadow-2xl scale-105' : ''
                  }`}
                  hover={!plan.highlight}
                >
                  {plan.highlight && (
                    <div className="absolute -top-4 left-1/2 -translate-x-1/2">
                      <span className="px-4 py-1 rounded-full bg-primary text-white text-sm font-medium shadow-lg">
                        Most Popular
                      </span>
                    </div>
                  )}

                  <div className="space-y-6">
                    {/* Icon */}
                    <div className={`p-3 rounded-xl bg-${plan.color}-500/10 w-fit`}>
                      <plan.icon className={`w-8 h-8 text-${plan.color}-600`} />
                    </div>

                    {/* Plan Name */}
                    <div>
                      <h3 className="text-2xl font-bold mb-2">{plan.name}</h3>
                      <p className="text-slate-600">{plan.description}</p>
                    </div>

                    {/* Price */}
                    <div className="space-y-2">
                      {plan.monthlyPrice ? (
                        <>
                          <div className="flex items-baseline gap-2">
                            <span className="text-4xl font-bold">
                              ₹{billingCycle === 'monthly'
                                ? plan.monthlyPrice.toLocaleString()
                                : Math.round(plan.annualPrice! / 12).toLocaleString()}
                            </span>
                            <span className="text-slate-600">/month</span>
                          </div>
                          {billingCycle === 'annual' && (
                            <p className="text-sm text-green-600">
                              Billed annually at ₹{plan.annualPrice!.toLocaleString()}
                            </p>
                          )}
                        </>
                      ) : (
                        <div>
                          <div className="text-4xl font-bold">Custom</div>
                          <p className="text-sm text-slate-600 mt-2">Contact us for pricing</p>
                        </div>
                      )}
                    </div>

                    {/* CTA Button */}
                    <Link href={plan.monthlyPrice ? "/register" : "/contact"} className="block">
                      <Button
                        className={`w-full ${plan.highlight ? 'hover-glow' : ''}`}
                        variant={plan.highlight ? 'default' : 'outline'}
                        size="lg"
                      >
                        {plan.monthlyPrice ? 'Start Free Trial' : 'Contact Sales'}
                        <ArrowRight className="w-4 h-4 ml-2" />
                      </Button>
                    </Link>

                    {/* Features */}
                    <div className="space-y-3 pt-6 border-t">
                      {plan.features.map((feature, i) => (
                        <div key={i} className="flex items-start gap-3">
                          <Check className="w-5 h-5 text-primary flex-shrink-0 mt-0.5" />
                          <span className="text-sm text-slate-700">{feature}</span>
                        </div>
                      ))}
                      {plan.notIncluded.map((feature, i) => (
                        <div key={i} className="flex items-start gap-3 opacity-40">
                          <X className="w-5 h-5 text-slate-400 flex-shrink-0 mt-0.5" />
                          <span className="text-sm text-slate-500 line-through">{feature}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                </GlassCard>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* FAQ Section */}
      <section className="section-padding bg-slate-50">
        <div className="container-wide">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl font-bold mb-4">
              Frequently Asked <span className="gradient-text">Questions</span>
            </h2>
          </motion.div>

          <div className="grid md:grid-cols-2 gap-6 max-w-4xl mx-auto">
            {[
              {
                q: "Can I switch plans anytime?",
                a: "Yes! You can upgrade or downgrade your plan at any time. Changes take effect immediately.",
              },
              {
                q: "Is there a free trial?",
                a: "Yes, all plans come with a 14-day free trial. No credit card required.",
              },
              {
                q: "What payment methods do you accept?",
                a: "We accept all major credit cards, UPI, net banking, and bank transfers for annual plans.",
              },
              {
                q: "Is my data secure?",
                a: "Absolutely. We're ISO 27001 certified and HIPAA compliant. Your data is encrypted at rest and in transit.",
              },
            ].map((faq, i) => (
              <GlassCard key={i} className="p-6">
                <h3 className="font-semibold text-lg mb-2">{faq.q}</h3>
                <p className="text-slate-600">{faq.a}</p>
              </GlassCard>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="section-padding relative overflow-hidden">
        <div className="absolute inset-0 gradient-mesh opacity-40" />

        <div className="container-wide relative">
          <GlassCard variant="strong" className="p-12 md:p-16 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="space-y-6 max-w-2xl mx-auto"
            >
              <Shield className="w-12 h-12 text-primary mx-auto" />

              <h2 className="text-4xl font-bold">
                Need a Custom Solution?
              </h2>

              <p className="text-xl text-slate-600">
                Enterprise plans with dedicated support, custom integrations, and flexible pricing.
              </p>

              <div className="flex flex-col sm:flex-row gap-4 justify-center pt-6">
                <Link href="/contact">
                  <Button size="lg" className="hover-glow">
                    Contact Sales
                  </Button>
                </Link>
                <Link href="/demo">
                  <Button size="lg" variant="outline" className="hover-lift">
                    Schedule Demo
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
