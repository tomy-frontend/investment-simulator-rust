// 複利計算: 毎月の積立で目標金額に到達するための月額を計算
fn calculate_monthly_investment_for_target(
    target_amount: f64,
    annual_rate: f64,
    years: usize,
) -> f64 {
    let months = years * 12;
    let monthly_rate = annual_rate / 12.0;

    // 将来価値の年金現在価値の逆算
    // FV = PMT × [(1 + r)^n - 1] / r
    // PMT = FV × r / [(1 + r)^n - 1]

    let denominator = ((1.0 + monthly_rate).powi(months as i32) - 1.0) / monthly_rate;
    target_amount / denominator
}

// 実際にシミュレーション（年利固定）
fn simulate_index_investment(
    monthly_investment: f64,
    annual_rate: f64,
    years: usize,
) -> Vec<f64> {
    let months = years * 12;
    let monthly_rate = annual_rate / 12.0;
    let mut wealth = 0.0;
    let mut yearly_wealth = Vec::with_capacity(years);

    for month in 1..=months {
        // 毎月の積立
        wealth += monthly_investment;

        // 月次の利息
        wealth *= 1.0 + monthly_rate;

        // 年末の資産を記録
        if month % 12 == 0 {
            yearly_wealth.push(wealth);
        }
    }

    yearly_wealth
}

fn format_yen(amount: f64) -> String {
    if amount >= 100_000_000.0 {
        format!("{:.2}億円", amount / 100_000_000.0)
    } else if amount >= 10_000.0 {
        format!("{:.1}万円", amount / 10_000.0)
    } else {
        format!("{:.0}円", amount)
    }
}

fn main() {
    let target_amount = 100_000_000.0; // 目標1億円
    let annual_rate = 0.05; // 年利5%
    let current_monthly = 50_000.0; // 現在の月額投資

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        目標資産1億円達成シミュレーター                       ║");
    println!("║        (インデックス投資 - 年利5%固定)                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("🎯 目標資産: {}", format_yen(target_amount));
    println!("📊 想定年利: {:.1}% (インデックス投資の長期平均)", annual_rate * 100.0);
    println!("💰 現在の月額投資: {}\n", format_yen(current_monthly));

    // 期間ごとの必要月額を計算
    let periods = vec![10, 15, 20, 25, 30];

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 目標1億円達成に必要な毎月の投資額");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("{:<8} {:<18} {:<18} {:<15} {:<12}",
             "期間", "必要月額", "総投資額(元本)", "運用益", "達成可否");
    println!("{}", "─".repeat(80));

    for &years in &periods {
        let required_monthly = calculate_monthly_investment_for_target(
            target_amount,
            annual_rate,
            years,
        );

        let total_invested = required_monthly * (years * 12) as f64;
        let profit = target_amount - total_invested;
        let profit_rate = (profit / total_invested) * 100.0;

        // 現在の投資額で達成可能かチェック
        let achievable = if required_monthly <= current_monthly {
            "✅ 達成可能"
        } else {
            "❌ 足りないよ！"
        };

        println!("{:>6}年 {:>18} {:>18} {:>15} ({:.0}%) {}",
            years,
            format_yen(required_monthly),
            format_yen(total_invested),
            format_yen(profit),
            profit_rate,
            achievable
        );
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 現在の投資額（{}）で到達できる資産額", format_yen(current_monthly));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("{:<8} {:<18} {:<18} {:<18}",
             "期間", "最終資産", "総投資額(元本)", "運用益");
    println!("{}", "─".repeat(75));

    for &years in &periods {
        let yearly_wealth = simulate_index_investment(current_monthly, annual_rate, years);
        let final_wealth = *yearly_wealth.last().unwrap_or(&0.0);
        let total_invested = current_monthly * (years * 12) as f64;
        let profit = final_wealth - total_invested;
        let profit_rate = (profit / total_invested) * 100.0;

        println!("{:>6}年 {:>18} {:>18} {:>18} ({:.0}%)",
            years,
            format_yen(final_wealth),
            format_yen(total_invested),
            format_yen(profit),
            profit_rate
        );
    }

    // 年次推移を表示（現在の投資額で30年間）
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 年次資産推移（毎月{}で30年間投資した場合）", format_yen(current_monthly));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let yearly_wealth = simulate_index_investment(current_monthly, annual_rate, 30);

    println!("{:<8} {:<18} {:<18} {:<18}",
             "経過年数", "資産額", "投資額(累計)", "運用益");
    println!("{}", "─".repeat(75));

    for (year, &wealth) in yearly_wealth.iter().enumerate() {
        let year_num = year + 1;
        let total_invested = current_monthly * (year_num * 12) as f64;
        let profit = wealth - total_invested;

        // 5年ごと、または1億円到達時、または最終年に表示
        if year_num % 5 == 0 || wealth >= target_amount || year_num == 30 {
            let marker = if wealth >= target_amount { "🎯" } else { "  " };
            println!("{}{:>6}年 {:>18} {:>18} {:>18}",
                marker,
                year_num,
                format_yen(wealth),
                format_yen(total_invested),
                format_yen(profit)
            );
        }
    }

    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  まとめ                                                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("• 年利5%のインデックス投資（S&P500など）を想定");
    println!("• 複利効果により、長期投資ほど有利");
    println!("• 現在の投資額（{}）を継続した場合:", format_yen(current_monthly));

    let final_30y = *simulate_index_investment(current_monthly, annual_rate, 30).last().unwrap();
    if final_30y >= target_amount {
        println!("  → 30年で目標1億円を達成可能！ 🎉");
    } else {
        let shortfall = target_amount - final_30y;
        println!("  → 30年後は約{} (目標まであと{})",
                 format_yen(final_30y),
                 format_yen(shortfall));

        // 必要な追加投資額を計算
        let required_for_30y = calculate_monthly_investment_for_target(target_amount, annual_rate, 30);
        let additional_needed = required_for_30y - current_monthly;
        println!("  → 目標達成には月額あと{}の追加投資が必要", format_yen(additional_needed));
    }

    println!("\n💡 ポイント:");
    println!("  • 早く始めるほど複利効果が大きい");
    println!("  • 長期投資（20年以上）が推奨");
    println!("  • 実際の年利は変動するため、余裕を持った計画を\n");
}
